/*
Copyright (c) 2020, R. Ou <rqou@robertou.com> and contributors
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice,
   this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

pub trait BitPattern<T> where Self: Sized {
    type BitsArrType;
    const BITS_COUNT: usize;

    type ErrType;

    type EncodeExtraType;
    type DecodeExtraType;

    const VARIANT_COUNT: usize;

    fn encode(&self, extra_data: Self::EncodeExtraType) -> Self::BitsArrType;
    fn decode(bits: &[bool], extra_data: Self::DecodeExtraType) -> Result<Self, Self::ErrType>;
    fn _pos_to_name(pos: usize) -> &'static str;
    fn _name_to_pos(name: &'static str) -> usize;

    fn variantname(var: usize) -> &'static str;
    fn variantdesc(var: usize) -> &'static str;
    fn variantbits(var: usize) -> &'static str;

    fn docs_as_ascii_table() -> String
    {
        let mut ret = String::new();

        let mut max_name_len = 0;
        for varname_i in 0..Self::VARIANT_COUNT {
            let varname = Self::variantname(varname_i);
            if varname.len() > max_name_len {
                max_name_len = varname.len()
            }
        }

        let mut max_desc_len = 0;
        for vardesc_i in 0..Self::VARIANT_COUNT {
            let vardesc = Self::variantdesc(vardesc_i);
            if vardesc.len() > max_desc_len {
                max_desc_len = vardesc.len()
            }
        }

        // Header
        for x in 0..Self::BITS_COUNT {
            ret.push_str(Self::_pos_to_name(x));
        }
        ret.push_str(" | ");
        for _ in 0..max_name_len {
            ret.push_str(" ");
        }
        ret.push_str(" |\n");

        // Separator
        for _ in 0..Self::BITS_COUNT {
            ret.push_str("-");
        }
        ret.push_str("-+-");
        for _ in 0..max_name_len {
            ret.push_str("-");
        }
        ret.push_str("-+-");
        for _ in 0..max_desc_len {
            ret.push_str("-");
        }
        ret.push_str("\n");

        // Data
        for i in 0..Self::VARIANT_COUNT {
            ret.push_str(Self::variantbits(i));
            ret.push_str(" | ");
            ret.push_str(Self::variantname(i));
            for _ in Self::variantname(i).len()..max_name_len {
                ret.push_str(" ");
            }
            ret.push_str(" | ");
            ret.push_str(Self::variantdesc(i));
            ret.push_str("\n");
        }

        ret
    }
}

impl BitPattern<()> for bool {
    type BitsArrType = [bool; 1];
    const BITS_COUNT: usize = 1;

    type ErrType = ();

    type EncodeExtraType = ();
    type DecodeExtraType = ();

    const VARIANT_COUNT: usize = 2;

    #[inline]
    fn encode(&self, _extra_data: Self::EncodeExtraType) -> Self::BitsArrType {
        [*self]
    }

    #[inline]
    fn decode(bits: &[bool], _extra_data: Self::DecodeExtraType) -> Result<Self, Self::ErrType> {
        Ok(bits[0])
    }

    #[inline]
    fn _pos_to_name(pos: usize) -> &'static str {
        ["0"][pos]
    }

    #[inline]
    fn _name_to_pos(name: &'static str) -> usize {
        match name {
            "0" => 0,
            _ => unreachable!()
        }
    }

    #[inline]
    fn variantname(var: usize) -> &'static str {
        ["false", "true"][var]
    }

    #[inline]
    fn variantdesc(var: usize) -> &'static str {
        ["false", "true"][var]
    }

    #[inline]
    fn variantbits(var: usize) -> &'static str {
        ["0", "1"][var]
    }
}
