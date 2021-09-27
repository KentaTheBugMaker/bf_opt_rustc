pub fn bf_main<R: std::io::Read, W: std::io::Write>(
    mut reader: R,
    mut writer: W,
) {
    let mut mem:Vec<u8> = vec![0;30000];
    let mut data_ptr:usize = 0;
    while mem[data_ptr]!=0{
    }
    data_ptr = data_ptr.wrapping_add(30);
    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
    data_ptr = data_ptr.wrapping_sub(9);
    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
    while mem[data_ptr]!=0{
        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
        while mem[data_ptr] != 0 { data_ptr += 10;}
        mem[data_ptr] = 0;
        data_ptr = data_ptr.wrapping_sub(10);
        while mem[data_ptr]!=0{
            mem[data_ptr + 10] += mem[data_ptr].wrapping_mul(1);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_sub(10);
        }
        data_ptr = data_ptr.wrapping_add(10);
        reader.read_exact(&mut mem[data_ptr..data_ptr + 1]).unwrap();
        mem[data_ptr] = mem[data_ptr].wrapping_sub(10);
    }
    data_ptr = data_ptr.wrapping_add(10);
    while mem[data_ptr]!=0{
        mem[data_ptr] = mem[data_ptr].wrapping_sub(37);
        data_ptr = data_ptr.wrapping_add(9);
        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
        data_ptr = data_ptr.wrapping_add(1);
    }
    data_ptr = data_ptr.wrapping_sub(1);
    while mem[data_ptr]!=0{
        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
        data_ptr = data_ptr.wrapping_add(1);
        while mem[data_ptr]!=0{
            data_ptr = data_ptr.wrapping_add(9);
            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
            data_ptr = data_ptr.wrapping_add(1);
        }
        data_ptr = data_ptr.wrapping_sub(1);
        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
        data_ptr = data_ptr.wrapping_sub(10);
    }
    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
    while mem[data_ptr]!=0{
        data_ptr = data_ptr.wrapping_add(1);
        mem[data_ptr] = mem[data_ptr].wrapping_add(48);
        writer.write_all(&mem[data_ptr..data_ptr + 1]).unwrap();
        mem[data_ptr] = mem[data_ptr].wrapping_sub(48);
        data_ptr = data_ptr.wrapping_sub(11);
    }
    mem[data_ptr] = mem[data_ptr].wrapping_add(58);
    writer.write_all(&mem[data_ptr..data_ptr + 1]).unwrap();
    mem[data_ptr] = mem[data_ptr].wrapping_sub(26);
    writer.write_all(&mem[data_ptr..data_ptr + 1]).unwrap();
    mem[data_ptr] = 0;
    data_ptr = data_ptr.wrapping_add(12);
    mem[data_ptr] = mem[data_ptr].wrapping_add(2);
    data_ptr = data_ptr.wrapping_sub(4);
    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
    while mem[data_ptr]!=0{
        mem[data_ptr] = 0;
        data_ptr = data_ptr.wrapping_add(2);
        while mem[data_ptr]!=0{
            data_ptr = data_ptr.wrapping_add(4);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_add(1);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_add(1);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_add(1);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_add(1);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_add(1);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_sub(7);
            while mem[data_ptr]!=0{
                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                data_ptr = data_ptr.wrapping_add(3);
                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                data_ptr = data_ptr.wrapping_add(1);
                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                data_ptr = data_ptr.wrapping_sub(4);
            }
            data_ptr = data_ptr.wrapping_add(8);
        }
        data_ptr = data_ptr.wrapping_sub(10);
        while mem[data_ptr]!=0{
            data_ptr = data_ptr.wrapping_add(6);
            mem[data_ptr - 4] += mem[data_ptr].wrapping_mul(1);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_sub(16);
        }
        data_ptr = data_ptr.wrapping_add(10);
        while mem[data_ptr]!=0{
            data_ptr = data_ptr.wrapping_add(1);
            while mem[data_ptr]!=0{
                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                data_ptr = data_ptr.wrapping_add(3);
                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                data_ptr = data_ptr.wrapping_add(2);
                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                data_ptr = data_ptr.wrapping_sub(5);
            }
            data_ptr = data_ptr.wrapping_add(9);
        }
        data_ptr = data_ptr.wrapping_sub(10);
        while mem[data_ptr]!=0{
            data_ptr = data_ptr.wrapping_add(6);
            mem[data_ptr - 5] += mem[data_ptr].wrapping_mul(1);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_sub(16);
        }
        data_ptr = data_ptr.wrapping_add(10);
        while mem[data_ptr]!=0{
            data_ptr = data_ptr.wrapping_add(3);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_add(3);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_add(1);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_add(3);
        }
        data_ptr = data_ptr.wrapping_sub(10);
        while mem[data_ptr] != 0 { data_ptr -= 10;}
        data_ptr = data_ptr.wrapping_add(9);
        mem[data_ptr] = 0;
        data_ptr = data_ptr.wrapping_add(7);
        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
        data_ptr = data_ptr.wrapping_sub(8);
        mem[data_ptr] = 0;
        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
        while mem[data_ptr]!=0{
            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
            data_ptr = data_ptr.wrapping_add(2);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(6);
                mem[data_ptr + 1] += mem[data_ptr].wrapping_mul(2);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_add(4);
            }
            data_ptr = data_ptr.wrapping_sub(10);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(8);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_add(1);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_sub(4);
                mem[data_ptr + 3] += mem[data_ptr].wrapping_mul(2);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_sub(15);
            }
            data_ptr = data_ptr.wrapping_add(10);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(8);
                while mem[data_ptr]!=0{
                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                    data_ptr = data_ptr.wrapping_add(1);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_sub(1);
                    while mem[data_ptr]!=0{
                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                        data_ptr = data_ptr.wrapping_add(1);
                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                        data_ptr = data_ptr.wrapping_sub(1);
                        while mem[data_ptr]!=0{
                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                            data_ptr = data_ptr.wrapping_add(1);
                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                            data_ptr = data_ptr.wrapping_sub(1);
                            while mem[data_ptr]!=0{
                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                data_ptr = data_ptr.wrapping_add(1);
                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                data_ptr = data_ptr.wrapping_sub(1);
                                while mem[data_ptr]!=0{
                                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                    data_ptr = data_ptr.wrapping_add(1);
                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                    data_ptr = data_ptr.wrapping_sub(1);
                                    while mem[data_ptr]!=0{
                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                        data_ptr = data_ptr.wrapping_add(1);
                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                        data_ptr = data_ptr.wrapping_sub(1);
                                        while mem[data_ptr]!=0{
                                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                            data_ptr = data_ptr.wrapping_add(1);
                                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                            data_ptr = data_ptr.wrapping_sub(1);
                                            while mem[data_ptr]!=0{
                                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                data_ptr = data_ptr.wrapping_add(1);
                                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                data_ptr = data_ptr.wrapping_sub(1);
                                                while mem[data_ptr]!=0{
                                                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                    data_ptr = data_ptr.wrapping_add(1);
                                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                    data_ptr = data_ptr.wrapping_sub(1);
                                                    while mem[data_ptr]!=0{
                                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                        data_ptr = data_ptr.wrapping_add(1);
                                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(9);
                                                        data_ptr = data_ptr.wrapping_add(9);
                                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                        data_ptr = data_ptr.wrapping_sub(10);
                                                        mem[data_ptr + 1] += mem[data_ptr].wrapping_mul(1);
                                                        mem[data_ptr] = 0;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                data_ptr = data_ptr.wrapping_add(2);
            }
            data_ptr = data_ptr.wrapping_sub(10);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(9);
                while mem[data_ptr]!=0{
                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                    data_ptr = data_ptr.wrapping_sub(1);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_sub(3);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_add(4);
                }
                data_ptr = data_ptr.wrapping_sub(19);
            }
            data_ptr = data_ptr.wrapping_add(10);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(7);
                while mem[data_ptr]!=0{
                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                    data_ptr = data_ptr.wrapping_sub(1);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_add(1);
                    while mem[data_ptr]!=0{
                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                        data_ptr = data_ptr.wrapping_sub(1);
                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                        data_ptr = data_ptr.wrapping_add(1);
                        while mem[data_ptr]!=0{
                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                            data_ptr = data_ptr.wrapping_sub(1);
                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                            data_ptr = data_ptr.wrapping_add(1);
                            while mem[data_ptr]!=0{
                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                data_ptr = data_ptr.wrapping_sub(1);
                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                data_ptr = data_ptr.wrapping_add(1);
                                while mem[data_ptr]!=0{
                                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                    data_ptr = data_ptr.wrapping_sub(1);
                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                    data_ptr = data_ptr.wrapping_add(1);
                                    while mem[data_ptr]!=0{
                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                        data_ptr = data_ptr.wrapping_sub(1);
                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                        data_ptr = data_ptr.wrapping_add(1);
                                        while mem[data_ptr]!=0{
                                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                            data_ptr = data_ptr.wrapping_sub(1);
                                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                            data_ptr = data_ptr.wrapping_add(1);
                                            while mem[data_ptr]!=0{
                                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                data_ptr = data_ptr.wrapping_sub(1);
                                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                data_ptr = data_ptr.wrapping_add(1);
                                                while mem[data_ptr]!=0{
                                                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                    data_ptr = data_ptr.wrapping_sub(1);
                                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                    data_ptr = data_ptr.wrapping_add(1);
                                                    while mem[data_ptr]!=0{
                                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                        data_ptr = data_ptr.wrapping_sub(1);
                                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(9);
                                                        data_ptr = data_ptr.wrapping_add(11);
                                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                        data_ptr = data_ptr.wrapping_sub(10);
                                                        mem[data_ptr - 1] += mem[data_ptr].wrapping_mul(1);
                                                        mem[data_ptr] = 0;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                data_ptr = data_ptr.wrapping_add(3);
            }
            data_ptr = data_ptr.wrapping_sub(10);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(4);
                while mem[data_ptr]!=0{
                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                    data_ptr = data_ptr.wrapping_add(3);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_add(2);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_sub(5);
                }
                data_ptr = data_ptr.wrapping_sub(14);
            }
            data_ptr = data_ptr.wrapping_add(10);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(7);
                mem[data_ptr - 3] += mem[data_ptr].wrapping_mul(1);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_add(3);
            }
            data_ptr = data_ptr.wrapping_sub(10);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(8);
                mem[data_ptr + 1] -= mem[data_ptr].wrapping_mul(1);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_add(1);
                while mem[data_ptr]!=0{
                    data_ptr = data_ptr.wrapping_sub(9);
                    while mem[data_ptr]!=0{
                        data_ptr = data_ptr.wrapping_sub(1);
                        mem[data_ptr] = 0;
                        data_ptr = data_ptr.wrapping_add(10);
                        mem[data_ptr - 10] += mem[data_ptr].wrapping_mul(1);
                        mem[data_ptr] = 0;
                        data_ptr = data_ptr.wrapping_sub(19);
                    }
                    data_ptr = data_ptr.wrapping_add(19);
                }
                data_ptr = data_ptr.wrapping_sub(19);
            }
            data_ptr = data_ptr.wrapping_add(9);
            while mem[data_ptr]!=0{
                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                while mem[data_ptr]!=0{
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    while mem[data_ptr]!=0{
                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                        while mem[data_ptr]!=0{
                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                            while mem[data_ptr]!=0{
                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                while mem[data_ptr]!=0{
                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                    while mem[data_ptr]!=0{
                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                        while mem[data_ptr]!=0{
                                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                            while mem[data_ptr]!=0{
                                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                while mem[data_ptr]!=0{
                                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                    while mem[data_ptr]!=0{
                                                        mem[data_ptr] = 0;
                                                        data_ptr = data_ptr.wrapping_sub(1);
                                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                        data_ptr = data_ptr.wrapping_add(1);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            data_ptr = data_ptr.wrapping_sub(1);
        }
        data_ptr = data_ptr.wrapping_add(8);
        while mem[data_ptr]!=0{
            data_ptr = data_ptr.wrapping_sub(6);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(8);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_add(1);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_sub(5);
                while mem[data_ptr]!=0{
                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                    data_ptr = data_ptr.wrapping_add(3);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_add(1);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_sub(4);
                }
                data_ptr = data_ptr.wrapping_add(6);
            }
            data_ptr = data_ptr.wrapping_sub(10);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(8);
                mem[data_ptr - 4] += mem[data_ptr].wrapping_mul(1);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_sub(3);
                while mem[data_ptr]!=0{
                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                    data_ptr = data_ptr.wrapping_add(3);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_add(1);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_sub(4);
                }
                data_ptr = data_ptr.wrapping_sub(15);
            }
            data_ptr = data_ptr.wrapping_add(10);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(9);
                mem[data_ptr - 4] += mem[data_ptr].wrapping_mul(1);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_add(1);
            }
            data_ptr = data_ptr.wrapping_sub(10);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(8);
                mem[data_ptr - 1] -= mem[data_ptr].wrapping_mul(1);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_sub(18);
            }
            data_ptr = data_ptr.wrapping_add(10);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(7);
                while mem[data_ptr]!=0{
                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                    data_ptr = data_ptr.wrapping_add(1);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_sub(1);
                    while mem[data_ptr]!=0{
                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                        data_ptr = data_ptr.wrapping_add(1);
                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                        data_ptr = data_ptr.wrapping_sub(1);
                        while mem[data_ptr]!=0{
                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                            data_ptr = data_ptr.wrapping_add(1);
                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                            data_ptr = data_ptr.wrapping_sub(1);
                            while mem[data_ptr]!=0{
                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                data_ptr = data_ptr.wrapping_add(1);
                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                data_ptr = data_ptr.wrapping_sub(1);
                                while mem[data_ptr]!=0{
                                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                    data_ptr = data_ptr.wrapping_add(1);
                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                    data_ptr = data_ptr.wrapping_sub(1);
                                    while mem[data_ptr]!=0{
                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                        data_ptr = data_ptr.wrapping_add(1);
                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                        data_ptr = data_ptr.wrapping_sub(1);
                                        while mem[data_ptr]!=0{
                                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                            data_ptr = data_ptr.wrapping_add(1);
                                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                            data_ptr = data_ptr.wrapping_sub(1);
                                            while mem[data_ptr]!=0{
                                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                data_ptr = data_ptr.wrapping_add(1);
                                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                data_ptr = data_ptr.wrapping_sub(1);
                                                while mem[data_ptr]!=0{
                                                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                    data_ptr = data_ptr.wrapping_add(1);
                                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                    data_ptr = data_ptr.wrapping_sub(1);
                                                    while mem[data_ptr]!=0{
                                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                        data_ptr = data_ptr.wrapping_add(1);
                                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                        data_ptr = data_ptr.wrapping_sub(1);
                                                        while mem[data_ptr]!=0{
                                                            mem[data_ptr] = mem[data_ptr].wrapping_add(10);
                                                            while mem[data_ptr]!=0{
                                                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                                data_ptr = data_ptr.wrapping_add(1);
                                                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                                data_ptr = data_ptr.wrapping_sub(1);
                                                            }
                                                            data_ptr = data_ptr.wrapping_add(10);
                                                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                            data_ptr = data_ptr.wrapping_sub(10);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                data_ptr = data_ptr.wrapping_add(3);
            }
            data_ptr = data_ptr.wrapping_add(7);
            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
            while mem[data_ptr]!=0{
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_sub(17);
                while mem[data_ptr]!=0{
                    data_ptr = data_ptr.wrapping_add(4);
                    mem[data_ptr] = 0;
                    data_ptr = data_ptr.wrapping_add(4);
                    mem[data_ptr - 4] += mem[data_ptr].wrapping_mul(1);
                    mem[data_ptr] = 0;
                    data_ptr = data_ptr.wrapping_sub(2);
                    mem[data_ptr + 2] += mem[data_ptr].wrapping_mul(1);
                    mem[data_ptr] = 0;
                    data_ptr = data_ptr.wrapping_sub(16);
                }
                data_ptr = data_ptr.wrapping_add(10);
                while mem[data_ptr]!=0{
                    data_ptr = data_ptr.wrapping_add(8);
                    while mem[data_ptr]!=0{
                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                        data_ptr = data_ptr.wrapping_add(1);
                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                        data_ptr = data_ptr.wrapping_sub(3);
                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                        data_ptr = data_ptr.wrapping_add(2);
                    }
                    data_ptr = data_ptr.wrapping_add(2);
                }
                data_ptr = data_ptr.wrapping_sub(10);
                while mem[data_ptr]!=0{
                    data_ptr = data_ptr.wrapping_add(3);
                    mem[data_ptr + 6] += mem[data_ptr].wrapping_mul(1);
                    mem[data_ptr] = 0;
                    data_ptr = data_ptr.wrapping_sub(13);
                }
                data_ptr = data_ptr.wrapping_add(10);
                while mem[data_ptr]!=0{
                    data_ptr = data_ptr.wrapping_add(9);
                    while mem[data_ptr]!=0{
                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                        data_ptr = data_ptr.wrapping_sub(6);
                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                        data_ptr = data_ptr.wrapping_add(6);
                        while mem[data_ptr]!=0{
                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                            data_ptr = data_ptr.wrapping_sub(6);
                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                            data_ptr = data_ptr.wrapping_add(6);
                            while mem[data_ptr]!=0{
                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                data_ptr = data_ptr.wrapping_sub(6);
                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                data_ptr = data_ptr.wrapping_add(6);
                                while mem[data_ptr]!=0{
                                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                    data_ptr = data_ptr.wrapping_sub(6);
                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                    data_ptr = data_ptr.wrapping_add(6);
                                    while mem[data_ptr]!=0{
                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                        data_ptr = data_ptr.wrapping_sub(6);
                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                        data_ptr = data_ptr.wrapping_add(6);
                                        while mem[data_ptr]!=0{
                                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                            data_ptr = data_ptr.wrapping_sub(6);
                                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                            data_ptr = data_ptr.wrapping_add(6);
                                            while mem[data_ptr]!=0{
                                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                data_ptr = data_ptr.wrapping_sub(6);
                                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                data_ptr = data_ptr.wrapping_add(6);
                                                while mem[data_ptr]!=0{
                                                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                    data_ptr = data_ptr.wrapping_sub(6);
                                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                    data_ptr = data_ptr.wrapping_add(6);
                                                    while mem[data_ptr]!=0{
                                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                        data_ptr = data_ptr.wrapping_sub(6);
                                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                        data_ptr = data_ptr.wrapping_add(6);
                                                        while mem[data_ptr]!=0{
                                                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                            data_ptr = data_ptr.wrapping_sub(6);
                                                            mem[data_ptr] = mem[data_ptr].wrapping_sub(9);
                                                            data_ptr = data_ptr.wrapping_add(16);
                                                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                            data_ptr = data_ptr.wrapping_sub(10);
                                                            mem[data_ptr - 6] += mem[data_ptr].wrapping_mul(1);
                                                            mem[data_ptr] = 0;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    data_ptr = data_ptr.wrapping_add(1);
                }
                data_ptr = data_ptr.wrapping_add(7);
            }
            data_ptr = data_ptr.wrapping_sub(17);
            while mem[data_ptr] != 0 { data_ptr -= 10;}
            data_ptr = data_ptr.wrapping_add(10);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(8);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_sub(2);
                mem[data_ptr + 1] += mem[data_ptr].wrapping_mul(1);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_sub(1);
                mem[data_ptr + 3] += mem[data_ptr].wrapping_mul(1);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_add(5);
            }
            data_ptr = data_ptr.wrapping_sub(10);
            while mem[data_ptr]!=0{
                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                data_ptr = data_ptr.wrapping_add(7);
                while mem[data_ptr]!=0{
                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                    data_ptr = data_ptr.wrapping_sub(7);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_add(7);
                    while mem[data_ptr]!=0{
                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                        data_ptr = data_ptr.wrapping_sub(7);
                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                        data_ptr = data_ptr.wrapping_add(6);
                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                        data_ptr = data_ptr.wrapping_add(1);
                        while mem[data_ptr]!=0{
                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                            data_ptr = data_ptr.wrapping_sub(7);
                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                            data_ptr = data_ptr.wrapping_add(7);
                            while mem[data_ptr]!=0{
                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                data_ptr = data_ptr.wrapping_sub(7);
                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                data_ptr = data_ptr.wrapping_add(6);
                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                data_ptr = data_ptr.wrapping_add(1);
                                while mem[data_ptr]!=0{
                                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                    data_ptr = data_ptr.wrapping_sub(7);
                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                    data_ptr = data_ptr.wrapping_add(7);
                                    while mem[data_ptr]!=0{
                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                        data_ptr = data_ptr.wrapping_sub(7);
                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                        data_ptr = data_ptr.wrapping_add(6);
                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                        data_ptr = data_ptr.wrapping_add(1);
                                        while mem[data_ptr]!=0{
                                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                            data_ptr = data_ptr.wrapping_sub(7);
                                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                            data_ptr = data_ptr.wrapping_add(7);
                                            while mem[data_ptr]!=0{
                                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                data_ptr = data_ptr.wrapping_sub(7);
                                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                data_ptr = data_ptr.wrapping_add(6);
                                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                data_ptr = data_ptr.wrapping_add(1);
                                                mem[data_ptr - 7] += mem[data_ptr].wrapping_mul(1);
                                                mem[data_ptr] = 0;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                data_ptr = data_ptr.wrapping_sub(7);
                mem[data_ptr + 7] += mem[data_ptr].wrapping_mul(1);
                mem[data_ptr] = 0;
                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                data_ptr = data_ptr.wrapping_sub(10);
            }
            data_ptr = data_ptr.wrapping_add(7);
            mem[data_ptr - 11] += mem[data_ptr].wrapping_mul(1);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_add(3);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(7);
                mem[data_ptr - 11] += mem[data_ptr].wrapping_mul(5);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_add(3);
            }
            data_ptr = data_ptr.wrapping_sub(10);
            while mem[data_ptr]!=0{
                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                data_ptr = data_ptr.wrapping_add(8);
                while mem[data_ptr]!=0{
                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                    data_ptr = data_ptr.wrapping_sub(8);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_add(8);
                    while mem[data_ptr]!=0{
                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                        data_ptr = data_ptr.wrapping_sub(8);
                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                        data_ptr = data_ptr.wrapping_add(5);
                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                        data_ptr = data_ptr.wrapping_add(3);
                        while mem[data_ptr]!=0{
                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                            data_ptr = data_ptr.wrapping_sub(8);
                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                            data_ptr = data_ptr.wrapping_add(8);
                            while mem[data_ptr]!=0{
                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                data_ptr = data_ptr.wrapping_sub(8);
                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                data_ptr = data_ptr.wrapping_add(5);
                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                data_ptr = data_ptr.wrapping_add(3);
                                while mem[data_ptr]!=0{
                                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                    data_ptr = data_ptr.wrapping_sub(8);
                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                    data_ptr = data_ptr.wrapping_add(8);
                                    while mem[data_ptr]!=0{
                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                        data_ptr = data_ptr.wrapping_sub(8);
                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                        data_ptr = data_ptr.wrapping_add(5);
                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                        data_ptr = data_ptr.wrapping_add(3);
                                        while mem[data_ptr]!=0{
                                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                            data_ptr = data_ptr.wrapping_sub(8);
                                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                            data_ptr = data_ptr.wrapping_add(8);
                                            while mem[data_ptr]!=0{
                                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                data_ptr = data_ptr.wrapping_sub(8);
                                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                data_ptr = data_ptr.wrapping_add(5);
                                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                data_ptr = data_ptr.wrapping_add(3);
                                                mem[data_ptr - 8] += mem[data_ptr].wrapping_mul(1);
                                                mem[data_ptr] = 0;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                data_ptr = data_ptr.wrapping_sub(8);
                mem[data_ptr + 8] += mem[data_ptr].wrapping_mul(1);
                mem[data_ptr] = 0;
                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                data_ptr = data_ptr.wrapping_sub(10);
            }
            data_ptr = data_ptr.wrapping_add(8);
            mem[data_ptr - 13] += mem[data_ptr].wrapping_mul(1);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_add(2);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(8);
                mem[data_ptr - 13] += mem[data_ptr].wrapping_mul(5);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_add(2);
            }
            data_ptr = data_ptr.wrapping_sub(10);
            while mem[data_ptr] != 0 { data_ptr -= 10;}
            data_ptr = data_ptr.wrapping_add(16);
        }
        data_ptr = data_ptr.wrapping_sub(6);
        while mem[data_ptr]!=0{
            data_ptr = data_ptr.wrapping_add(3);
            while mem[data_ptr]!=0{
                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                data_ptr = data_ptr.wrapping_add(4);
                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                data_ptr = data_ptr.wrapping_add(1);
                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                data_ptr = data_ptr.wrapping_sub(5);
            }
            data_ptr = data_ptr.wrapping_add(7);
        }
        data_ptr = data_ptr.wrapping_sub(10);
        while mem[data_ptr]!=0{
            data_ptr = data_ptr.wrapping_add(7);
            mem[data_ptr - 4] += mem[data_ptr].wrapping_mul(1);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_sub(5);
            while mem[data_ptr]!=0{
                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                data_ptr = data_ptr.wrapping_add(5);
                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                data_ptr = data_ptr.wrapping_add(2);
                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                data_ptr = data_ptr.wrapping_sub(7);
            }
            data_ptr = data_ptr.wrapping_sub(12);
        }
        data_ptr = data_ptr.wrapping_add(10);
        while mem[data_ptr]!=0{
            data_ptr = data_ptr.wrapping_add(7);
            mem[data_ptr - 5] += mem[data_ptr].wrapping_mul(1);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_add(3);
        }
        data_ptr = data_ptr.wrapping_sub(10);
        while mem[data_ptr]!=0{
            data_ptr = data_ptr.wrapping_add(9);
            mem[data_ptr - 1] -= mem[data_ptr].wrapping_mul(1);
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_sub(1);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_sub(8);
                while mem[data_ptr]!=0{
                    data_ptr = data_ptr.wrapping_sub(2);
                    mem[data_ptr] = 0;
                    data_ptr = data_ptr.wrapping_add(10);
                    mem[data_ptr - 10] += mem[data_ptr].wrapping_mul(1);
                    mem[data_ptr] = 0;
                    data_ptr = data_ptr.wrapping_sub(18);
                }
                data_ptr = data_ptr.wrapping_add(18);
            }
            data_ptr = data_ptr.wrapping_sub(18);
        }
        data_ptr = data_ptr.wrapping_add(8);
        while mem[data_ptr]!=0{
            data_ptr = data_ptr.wrapping_add(1);
            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
            data_ptr = data_ptr.wrapping_sub(1);
            while mem[data_ptr]!=0{
                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                while mem[data_ptr]!=0{
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    while mem[data_ptr]!=0{
                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                        while mem[data_ptr]!=0{
                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                            while mem[data_ptr]!=0{
                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                while mem[data_ptr]!=0{
                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                    while mem[data_ptr]!=0{
                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                        while mem[data_ptr]!=0{
                                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                            while mem[data_ptr]!=0{
                                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                while mem[data_ptr]!=0{
                                                    mem[data_ptr] = 0;
                                                    data_ptr = data_ptr.wrapping_add(1);
                                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                    data_ptr = data_ptr.wrapping_sub(1);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        data_ptr = data_ptr.wrapping_add(1);
        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
        while mem[data_ptr]!=0{
            mem[data_ptr] = 0;
            data_ptr = data_ptr.wrapping_sub(1);
            mem[data_ptr] = 0;
            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
            data_ptr = data_ptr.wrapping_add(4);
            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
            data_ptr = data_ptr.wrapping_add(8);
            while mem[data_ptr] != 0 { data_ptr += 10;}
            data_ptr = data_ptr.wrapping_sub(10);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_sub(6);
                while mem[data_ptr]!=0{
                    data_ptr = data_ptr.wrapping_sub(4);
                    while mem[data_ptr] != 0 { data_ptr -= 10;}
                    data_ptr = data_ptr.wrapping_add(4);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_sub(10);
                }
                data_ptr = data_ptr.wrapping_sub(4);
            }
            data_ptr = data_ptr.wrapping_add(20);
            while mem[data_ptr] != 0 { data_ptr += 10;}
            data_ptr = data_ptr.wrapping_sub(10);
            while mem[data_ptr] != 0 { data_ptr -= 10;}
            data_ptr = data_ptr.wrapping_add(4);
            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
            while mem[data_ptr]!=0{
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_add(8);
                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                data_ptr = data_ptr.wrapping_sub(2);
                while mem[data_ptr]!=0{
                    data_ptr = data_ptr.wrapping_add(1);
                    mem[data_ptr] = 0;
                    data_ptr = data_ptr.wrapping_add(2);
                    mem[data_ptr - 2] += mem[data_ptr].wrapping_mul(1);
                    mem[data_ptr] = 0;
                    data_ptr = data_ptr.wrapping_add(7);
                }
                data_ptr = data_ptr.wrapping_sub(10);
                while mem[data_ptr]!=0{
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_add(2);
                    while mem[data_ptr]!=0{
                        data_ptr = data_ptr.wrapping_add(8);
                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                        data_ptr = data_ptr.wrapping_add(2);
                    }
                    data_ptr = data_ptr.wrapping_sub(2);
                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                    data_ptr = data_ptr.wrapping_sub(10);
                }
                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                while mem[data_ptr]!=0{
                    data_ptr = data_ptr.wrapping_add(2);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(48);
                    writer.write_all(&mem[data_ptr..data_ptr + 1]).unwrap();
                    mem[data_ptr] = mem[data_ptr].wrapping_sub(48);
                    data_ptr = data_ptr.wrapping_sub(12);
                }
                mem[data_ptr] = mem[data_ptr].wrapping_add(32);
                writer.write_all(&mem[data_ptr..data_ptr + 1]).unwrap();
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_add(4);
            }
            data_ptr = data_ptr.wrapping_add(6);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(2);
                while mem[data_ptr]!=0{
                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                    data_ptr = data_ptr.wrapping_add(5);
                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                    data_ptr = data_ptr.wrapping_sub(5);
                    while mem[data_ptr]!=0{
                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                        data_ptr = data_ptr.wrapping_add(5);
                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                        data_ptr = data_ptr.wrapping_sub(5);
                        while mem[data_ptr]!=0{
                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                            data_ptr = data_ptr.wrapping_add(5);
                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                            data_ptr = data_ptr.wrapping_sub(5);
                            while mem[data_ptr]!=0{
                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                data_ptr = data_ptr.wrapping_add(5);
                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                data_ptr = data_ptr.wrapping_sub(5);
                                while mem[data_ptr]!=0{
                                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                    data_ptr = data_ptr.wrapping_add(5);
                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                    data_ptr = data_ptr.wrapping_sub(5);
                                    while mem[data_ptr]!=0{
                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                        data_ptr = data_ptr.wrapping_add(5);
                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                        data_ptr = data_ptr.wrapping_sub(5);
                                        while mem[data_ptr]!=0{
                                            mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                            data_ptr = data_ptr.wrapping_add(5);
                                            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                            data_ptr = data_ptr.wrapping_sub(5);
                                            while mem[data_ptr]!=0{
                                                mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                data_ptr = data_ptr.wrapping_add(5);
                                                mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                data_ptr = data_ptr.wrapping_sub(5);
                                                while mem[data_ptr]!=0{
                                                    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                    data_ptr = data_ptr.wrapping_add(5);
                                                    mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                    data_ptr = data_ptr.wrapping_sub(5);
                                                    while mem[data_ptr]!=0{
                                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
                                                        data_ptr = data_ptr.wrapping_add(5);
                                                        mem[data_ptr] = mem[data_ptr].wrapping_sub(9);
                                                        data_ptr = data_ptr.wrapping_add(5);
                                                        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
                                                        data_ptr = data_ptr.wrapping_sub(10);
                                                        mem[data_ptr + 5] += mem[data_ptr].wrapping_mul(1);
                                                        mem[data_ptr] = 0;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                data_ptr = data_ptr.wrapping_add(8);
            }
            data_ptr = data_ptr.wrapping_sub(10);
            while mem[data_ptr]!=0{
                data_ptr = data_ptr.wrapping_add(7);
                mem[data_ptr - 5] += mem[data_ptr].wrapping_mul(1);
                mem[data_ptr] = 0;
                data_ptr = data_ptr.wrapping_sub(17);
            }
            data_ptr = data_ptr.wrapping_add(9);
        }
        data_ptr = data_ptr.wrapping_sub(1);
    }
    data_ptr = data_ptr.wrapping_add(2);
    while mem[data_ptr] != 0 { data_ptr += 10;}
    data_ptr = data_ptr.wrapping_sub(10);
    while mem[data_ptr]!=0{
        mem[data_ptr] = mem[data_ptr].wrapping_add(1);
        data_ptr = data_ptr.wrapping_add(1);
        while mem[data_ptr]!=0{
            data_ptr = data_ptr.wrapping_add(9);
            mem[data_ptr] = mem[data_ptr].wrapping_add(1);
            data_ptr = data_ptr.wrapping_add(1);
        }
        data_ptr = data_ptr.wrapping_sub(1);
        mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
        data_ptr = data_ptr.wrapping_sub(10);
    }
    mem[data_ptr] = mem[data_ptr].wrapping_sub(1);
    while mem[data_ptr]!=0{
        data_ptr = data_ptr.wrapping_add(1);
        mem[data_ptr] = mem[data_ptr].wrapping_add(48);
        writer.write_all(&mem[data_ptr..data_ptr + 1]).unwrap();
        data_ptr = data_ptr.wrapping_sub(11);
    }
    mem[data_ptr] = mem[data_ptr].wrapping_add(10);
    writer.write_all(&mem[data_ptr..data_ptr + 1]).unwrap();
}