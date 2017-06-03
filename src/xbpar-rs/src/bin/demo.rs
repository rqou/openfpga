/***********************************************************************************************************************
 * Copyright (C) 2017 Robert Ou and contributors                                                                       *
 *                                                                                                                     *
 * This program is free software; you can redistribute it and/or modify it under the terms of the GNU Lesser General   *
 * Public License as published by the Free Software Foundation; either version 2.1 of the License, or (at your option) *
 * any later version.                                                                                                  *
 *                                                                                                                     *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied  *
 * warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for     *
 * more details.                                                                                                       *
 *                                                                                                                     *
 * You should have received a copy of the GNU Lesser General Public License along with this program; if not, you may   *
 * find one here:                                                                                                      *
 * https://www.gnu.org/licenses/old-licenses/lgpl-2.1.txt                                                              *
 * or you may search the http://www.gnu.org website for the version 2.1 license, or you may write to the Free Software *
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA                                      *
 **********************************************************************************************************************/

extern crate xbpar_rs;
use xbpar_rs::*;

use std::collections::HashMap;
use std::ptr;

struct TrivialPAREngine<'a> {
    base_engine: Option<&'a mut BasePAREngine>,
}

impl<'a> PAREngineImpl<'a> for TrivialPAREngine<'a> {
    fn set_base_engine<'b: 'a>(&'a mut self, base_engine: &'b mut BasePAREngine) {
        self.base_engine = Some(base_engine);
    }

    fn compute_and_print_score(&mut self, iteration: u32) -> (u32, Vec<&PARGraphEdge>) {
        println!("compute_and_print_score");
        let base_engine = self.base_engine.as_mut().unwrap();
        base_engine.compute_and_print_score(iteration)
    }

    fn sanity_check(&mut self) -> bool {
        println!("sanity_check");
        let base_engine = self.base_engine.as_mut().unwrap();
        base_engine.sanity_check()
    }

    fn initial_placement(&mut self) -> bool {
        println!("initial_placement");
        let base_engine = self.base_engine.as_mut().unwrap();
        base_engine.initial_placement()
    }

    fn initial_placement_core(&mut self) -> bool {
        println!("initial_placement_core");
        let base_engine = self.base_engine.as_mut().unwrap();
        //For each label, mate each node in the netlist with the first legal mate in the device.
        //Simple and deterministic.
        let (mut m_netlist, mut m_device) = base_engine.get_both_netlists_mut();
        let nmax_net = m_netlist.get_max_label();
        for label in 0..(nmax_net + 1)
        {
            let nnet = m_netlist.get_num_nodes_with_label(label);
            let nsites = m_device.get_num_nodes_with_label(label);

            let mut nsite = 0;
            for net in 0..nnet
            {
                let netnode = m_netlist.get_node_by_label_and_index_mut(label, net);

                //If the netlist node is already constrained, don't auto-place it
                if netnode.get_mate().is_some() {
                    continue;
                }

                //Try to find a legal site
                let mut found = false;
                while nsite < nsites
                {
                    let devnode = m_device.get_node_by_label_and_index_mut(label, nsite);
                    nsite += 1;

                    //If the site is used, we don't want to disturb what's already there
                    //because it was probably LOC'd
                    if devnode.get_mate().is_some() {
                        continue;
                    }

                    //Site is unused, mate with it
                    netnode.mate_with(devnode);
                    found = true;
                    break;
                }

                //This can happen in rare cases
                //(for example, we constrained all of the 8-bit counters to COUNT14 sites and now have a COUNT14).
                if !found
                {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    // Create a device graph with four nodes, two of type A and two of type B
    // Create a netlist graph with two nodes, one of type A and one of type B

    // The graphs
    let mut dgraph = PARGraph::new();
    let mut ngraph = PARGraph::new();

    // Labels
    let typea_label_d = dgraph.allocate_label();
    let typea_label_n = ngraph.allocate_label();
    assert_eq!(typea_label_d, typea_label_n);
    let typeb_label_d = dgraph.allocate_label();
    let typeb_label_n = ngraph.allocate_label();
    assert_eq!(typeb_label_d, typeb_label_n);
    let mut label_map = HashMap::new();
    label_map.insert(typea_label_d, "Type A node");
    label_map.insert(typeb_label_d, "Type B node");

    // Device graph
    let d_type_a_1 = dgraph.add_new_node(typea_label_d, ptr::null_mut());
    let d_type_a_2 = dgraph.add_new_node(typea_label_d, ptr::null_mut());
    let d_type_b_1 = dgraph.add_new_node(typeb_label_d, ptr::null_mut());
    let d_type_b_2 = dgraph.add_new_node(typeb_label_d, ptr::null_mut());

    {
        let (mut a1, b1) = dgraph.get_node_by_index_mut_pair(d_type_a_1, d_type_b_1);
        a1.add_edge("A to B", b1, "B to A");
    }
    {
        let (mut a1, b2) = dgraph.get_node_by_index_mut_pair(d_type_a_1, d_type_b_2);
        a1.add_edge("A to B", b2, "B to A");
    }
    {
        let (mut a2, b1) = dgraph.get_node_by_index_mut_pair(d_type_a_2, d_type_b_1);
        a2.add_edge("A to B", b1, "B to A");
    }
    {
        let (mut a2, b2) = dgraph.get_node_by_index_mut_pair(d_type_a_2, d_type_b_2);
        a2.add_edge("A to B", b2, "B to A");
    }

    // Netlist graph
    let n_type_a_1 = ngraph.add_new_node(typea_label_d, ptr::null_mut());
    let n_type_b_1 = ngraph.add_new_node(typeb_label_d, ptr::null_mut());

    {
        let (mut a1, b1) = ngraph.get_node_by_index_mut_pair(n_type_a_1, n_type_b_1);
        a1.add_edge("A to B", b1, "B to A");
    }

    // Do the thing!
    let engine_impl = TrivialPAREngine {
        base_engine: None,
    };
    let mut engine_obj = PAREngine::new(engine_impl, &mut ngraph, &mut dgraph);
    if !engine_obj.place_and_route(0) {
        panic!("PAR failed!");
    }
}
