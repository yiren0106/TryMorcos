mod macros {
    mod recall {
        #[macro_export]
        macro_rules! call_with_larch {
            ($callback:ident) => {
                $callback!("larch")
            };
        }
        #[macro_export]
        macro_rules! expand_to_larch {
            () => {
                "larch"
            };
        }
        #[macro_export]
        macro_rules! recognize_tree {
            ("larch") => {
                println!("#1, the Larch.")
            };
            (redwood) => {
                println!("#2, the Mighty Redwood.")
            };
            (fir) => {
                println!("#3, the Fir.")
            };
            (chestnut) => {
                println!("#4, the Horse Chestnut.")
            };
            (pine) => {
                println!("#5, the Scots Pine.")
            };
            ($($other:tt)*) => {
                println!("I don't know; some kind of birch maybe?")
            };
        }
    }
    mod vec {
        #[macro_export]
        macro_rules! vec_strs {
            ($($element:expr ),*) => {
                {
                    let mut v = Vec::new();
                    $(v.push(format!("{}", $element));)*
                    v
                }
            };
        }
    }
    mod fib {
        #[macro_export]
        macro_rules! ruc {
            (a[n]:$sty:ty = $($init:expr),+ ; ... ; $recur:expr) => {{
                use std::ops::Index;

                struct Recurrence {
                    mem: [$sty; 2],
                    pos: usize,
                }

                struct IndexOffset<'a> {
                    slice: &'a [$sty; 2],
                    offset: usize,
                }

                impl<'a> Index<usize> for IndexOffset<'a> {
                    type Output = $sty;

                    #[inline(always)]
                    fn index<'b>(&'b self, index: usize) -> &'b $sty {
                        use std::num::Wrapping;

                        let index = Wrapping(index);
                        let offset = Wrapping(self.offset);
                        let window = Wrapping(2);

                        let real_index = index - offset + window;
                        &self.slice[real_index.0]
                    }
                }

                impl Iterator for Recurrence {
                    type Item = $sty;

                    #[inline]
                    fn next(&mut self) -> Option<$sty> {
                        if self.pos < 2 {
                            let next_val = self.mem[self.pos];
                            self.pos += 1;
                            Some(next_val)
                        } else {
                            let next_val = {
                                let n = self.pos;
                                let a = IndexOffset {
                                    slice: &self.mem,
                                    offset: n,
                                };
                                (a[n - 1] + a[n - 2])
                            };

                            {
                                use std::mem::swap;

                                let mut swap_tmp = next_val;
                                for i in (0..2).rev() {
                                    swap(&mut swap_tmp, &mut self.mem[i]);
                                }
                            }

                            self.pos += 1;
                            Some(next_val)
                        }
                    }
                }

                Recurrence {
                    mem: [0, 1],
                    pos: 0,
                }
            }};
        }
    }
    mod counts {
        #[macro_export]
        macro_rules! count_tts_1 {
            () => {0usize};
            ($_head:tt $($tail:tt)*) => {1usize + count_tts_1!($($tail)*)};
        }
        #[macro_export]
        macro_rules! count_tts_2 {
            (@replace_expr ($_t:tt $sub:expr)) => {
                $sub
            };
            ($($tts:tt)*) => {0usize $(+  count_tts_2!(@replace_expr ($tts 1usize)))* };
        }
    }
}
