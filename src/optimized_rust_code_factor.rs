pub fn bf_main<R: std::io::Read, W: std::io::Write>(mut reader: R, mut writer: W) {
    let mut mem: Vec<u8> = vec![0; 30000];
    let mut ptr: usize = 0;
    //while mem[ptr] != 0 {}
    ptr = ptr.wrapping_add(30);
    mem[ptr] = mem[ptr].wrapping_sub(1);
    ptr = ptr.wrapping_sub(9);
    mem[ptr] = mem[ptr].wrapping_add(1);
    while mem[ptr] != 0 {
        mem[ptr] = mem[ptr].wrapping_sub(1);
        while mem[ptr] != 0 {
            ptr += 10;
        }
        //mem[ptr] = 0;
        ptr = ptr.wrapping_sub(10);
        while mem[ptr] != 0 {
            mem[ptr + 10] += mem[ptr];
            mem[ptr] = 0;
            ptr = ptr.wrapping_sub(10);
        }
        ptr = ptr.wrapping_add(10);
        reader.read_exact(&mut mem[ptr..ptr + 1]).unwrap();
        mem[ptr] = mem[ptr].wrapping_sub(10);
    }
    ptr = ptr.wrapping_add(10);
    while mem[ptr] != 0 {
        mem[ptr] = mem[ptr].wrapping_sub(37);
        ptr = ptr.wrapping_add(9);
        mem[ptr] = mem[ptr].wrapping_sub(1);
        ptr = ptr.wrapping_add(1);
    }
    ptr = ptr.wrapping_sub(1);
    while mem[ptr] != 0 {
        mem[ptr] = mem[ptr].wrapping_add(1);
        ptr = ptr.wrapping_add(1);
        while mem[ptr] != 0 {
            ptr = ptr.wrapping_add(9);
            mem[ptr] = mem[ptr].wrapping_add(1);
            ptr = ptr.wrapping_add(1);
        }
        ptr = ptr.wrapping_sub(1);
        mem[ptr] = mem[ptr].wrapping_sub(1);
        ptr = ptr.wrapping_sub(10);
    }
    mem[ptr] = mem[ptr].wrapping_sub(1);
    while mem[ptr] != 0 {
        ptr = ptr.wrapping_add(1);
        mem[ptr] = mem[ptr].wrapping_add(48);
        writer.write_all(&mem[ptr..ptr + 1]).unwrap();
        mem[ptr] = mem[ptr].wrapping_sub(48);
        ptr = ptr.wrapping_sub(11);
    }
    mem[ptr] = mem[ptr].wrapping_add(58);
    writer.write_all(&mem[ptr..ptr + 1]).unwrap();
    mem[ptr] = mem[ptr].wrapping_sub(26);
    writer.write_all(&mem[ptr..ptr + 1]).unwrap();
    mem[ptr] = 0;
    ptr = ptr.wrapping_add(12);
    mem[ptr] = mem[ptr].wrapping_add(2);
    ptr = ptr.wrapping_sub(4);
    mem[ptr] = mem[ptr].wrapping_add(1);
    while mem[ptr] != 0 {
        mem[ptr] = 0;
        ptr = ptr.wrapping_add(2);
        while mem[ptr] != 0 {
            ptr = ptr.wrapping_add(4);
            mem[ptr] = 0;
            ptr = ptr.wrapping_add(1);
            mem[ptr] = 0;
            ptr = ptr.wrapping_add(1);
            mem[ptr] = 0;
            ptr = ptr.wrapping_add(1);
            mem[ptr] = 0;
            ptr = ptr.wrapping_add(1);
            mem[ptr] = 0;
            ptr = ptr.wrapping_add(1);
            mem[ptr] = 0;
            ptr = ptr.wrapping_sub(7);
            mem[ptr + 3] += mem[ptr];
            mem[ptr + 4] += mem[ptr];
            mem[ptr] = 0;
            ptr = ptr.wrapping_add(8);
        }
        ptr = ptr.wrapping_sub(10);
        while mem[ptr] != 0 {
            ptr = ptr.wrapping_add(6);
            mem[ptr - 4] += mem[ptr];
            mem[ptr] = 0;
            ptr = ptr.wrapping_sub(16);
        }
        ptr = ptr.wrapping_add(10);
        while mem[ptr] != 0 {
            //ptr = ptr.wrapping_add(1);
            //mem[ptr + 3] += mem[ptr];
            mem[ptr+4] += mem[ptr+1];
            mem[ptr+6] += mem[ptr+1];
            //mem[ptr + 5] += mem[ptr];
            //mem[ptr] = 0;
            mem[ptr+1]=0;
            ptr = ptr.wrapping_add(10);
            //ptr = ptr.wrapping_add(9);
        }
        ptr = ptr.wrapping_sub(10);
        while mem[ptr] != 0 {
            ptr = ptr.wrapping_add(6);
            mem[ptr - 5] += mem[ptr];
            mem[ptr] = 0;
            ptr = ptr.wrapping_sub(16);
        }
        ptr = ptr.wrapping_add(10);
        while mem[ptr] != 0 {
            ptr = ptr.wrapping_add(3);
            mem[ptr] = 0;
            ptr = ptr.wrapping_add(3);
            mem[ptr] = 0;
            ptr = ptr.wrapping_add(1);
            mem[ptr] = 0;
            ptr = ptr.wrapping_add(3);
        }
        ptr = ptr.wrapping_sub(10);
        while mem[ptr] != 0 {
            ptr -= 10;
        }
        ptr = ptr.wrapping_add(9);
        mem[ptr] = 0;
        ptr = ptr.wrapping_add(7);
        mem[ptr] = mem[ptr].wrapping_add(1);
        ptr = ptr.wrapping_sub(8);
        mem[ptr] = 0;
        mem[ptr] = mem[ptr].wrapping_add(1);
        while mem[ptr] != 0 {
            mem[ptr] = mem[ptr].wrapping_sub(1);
            ptr = ptr.wrapping_add(2);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(6);
                mem[ptr + 1] += mem[ptr].wrapping_mul(2);
                mem[ptr] = 0;
                ptr = ptr.wrapping_add(4);
            }
            ptr = ptr.wrapping_sub(10);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(8);
                mem[ptr] = 0;
                ptr = ptr.wrapping_add(1);
                mem[ptr] = 0;
                ptr = ptr.wrapping_sub(4);
                mem[ptr + 3] += mem[ptr].wrapping_mul(2);
                mem[ptr] = 0;
                ptr = ptr.wrapping_sub(15);
            }
            ptr = ptr.wrapping_add(10);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(8);
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                    ptr = ptr.wrapping_add(1);
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr = ptr.wrapping_sub(1);
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr = ptr.wrapping_add(1);
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr = ptr.wrapping_sub(1);
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_sub(1);
                            ptr = ptr.wrapping_add(1);
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            ptr = ptr.wrapping_sub(1);
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr = ptr.wrapping_add(1);
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                ptr = ptr.wrapping_sub(1);
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                    ptr = ptr.wrapping_add(1);
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    ptr = ptr.wrapping_sub(1);
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr = ptr.wrapping_add(1);
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        ptr = ptr.wrapping_sub(1);
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                            ptr = ptr.wrapping_add(1);
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            ptr = ptr.wrapping_sub(1);
                                            while mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr = ptr.wrapping_add(1);
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                ptr = ptr.wrapping_sub(1);
                                                while mem[ptr] != 0 {
                                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                                    ptr = ptr.wrapping_add(1);
                                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                                    ptr = ptr.wrapping_sub(1);
                                                    while mem[ptr] != 0 {
                                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                                        ptr = ptr.wrapping_add(1);
                                                        mem[ptr] = mem[ptr].wrapping_sub(9);
                                                        ptr = ptr.wrapping_add(9);
                                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                                        ptr = ptr.wrapping_sub(10);
                                                        mem[ptr + 1] += mem[ptr];
                                                        mem[ptr] = 0;
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
                ptr = ptr.wrapping_add(2);
            }
            ptr = ptr.wrapping_sub(10);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(9);
                mem[ptr - 1] += mem[ptr];
                mem[ptr - 4] += mem[ptr];
                mem[ptr] = 0;
                ptr = ptr.wrapping_sub(19);
            }
            ptr = ptr.wrapping_add(10);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(7);
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                    ptr = ptr.wrapping_sub(1);
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr = ptr.wrapping_add(1);
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr = ptr.wrapping_sub(1);
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr = ptr.wrapping_add(1);
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_sub(1);
                            ptr = ptr.wrapping_sub(1);
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            ptr = ptr.wrapping_add(1);
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr = ptr.wrapping_sub(1);
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                ptr = ptr.wrapping_add(1);
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                    ptr = ptr.wrapping_sub(1);
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    ptr = ptr.wrapping_add(1);
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr = ptr.wrapping_sub(1);
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        ptr = ptr.wrapping_add(1);
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                            ptr = ptr.wrapping_sub(1);
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            ptr = ptr.wrapping_add(1);
                                            while mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr = ptr.wrapping_sub(1);
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                ptr = ptr.wrapping_add(1);
                                                while mem[ptr] != 0 {
                                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                                    ptr = ptr.wrapping_sub(1);
                                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                                    ptr = ptr.wrapping_add(1);
                                                    while mem[ptr] != 0 {
                                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                                        ptr = ptr.wrapping_sub(1);
                                                        mem[ptr] = mem[ptr].wrapping_sub(9);
                                                        ptr = ptr.wrapping_add(11);
                                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                                        ptr = ptr.wrapping_sub(10);
                                                        mem[ptr - 1] += mem[ptr];
                                                        mem[ptr] = 0;
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
                ptr = ptr.wrapping_add(3);
            }
            ptr = ptr.wrapping_sub(10);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(4);
                mem[ptr + 3] += mem[ptr];
                mem[ptr + 5] += mem[ptr];
                mem[ptr] = 0;
                ptr = ptr.wrapping_sub(14);
            }
            ptr = ptr.wrapping_add(10);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(7);
                mem[ptr - 3] += mem[ptr];
                mem[ptr] = 0;
                ptr = ptr.wrapping_add(3);
            }
            ptr = ptr.wrapping_sub(10);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(8);
                mem[ptr + 1] -= mem[ptr];
                mem[ptr] = 0;
                ptr = ptr.wrapping_add(1);
                while mem[ptr] != 0 {
                    ptr = ptr.wrapping_sub(9);
                    while mem[ptr] != 0 {
                        ptr = ptr.wrapping_sub(1);
                        mem[ptr] = 0;
                        ptr = ptr.wrapping_add(10);
                        mem[ptr - 10] += mem[ptr];
                        mem[ptr] = 0;
                        ptr = ptr.wrapping_sub(19);
                    }
                    ptr = ptr.wrapping_add(19);
                }
                ptr = ptr.wrapping_sub(19);
            }
            ptr = ptr.wrapping_add(9);
            while mem[ptr] != 0 {
                mem[ptr] = mem[ptr].wrapping_add(1);
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            while mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                while mem[ptr] != 0 {
                                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                                    while mem[ptr] != 0 {
                                                        mem[ptr] = 0;
                                                        ptr = ptr.wrapping_sub(1);
                                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                                        ptr = ptr.wrapping_add(1);
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
            ptr = ptr.wrapping_sub(1);
        }
        ptr = ptr.wrapping_add(8);
        while mem[ptr] != 0 {
            ptr = ptr.wrapping_sub(6);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(8);
                mem[ptr] = 0;
                ptr = ptr.wrapping_add(1);
                mem[ptr] = 0;
                ptr = ptr.wrapping_sub(5);
                mem[ptr + 3] += mem[ptr];
                mem[ptr + 4] += mem[ptr];
                mem[ptr] = 0;
                ptr = ptr.wrapping_add(6);
            }
            ptr = ptr.wrapping_sub(10);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(8);
                mem[ptr - 4] += mem[ptr];
                mem[ptr] = 0;
                ptr = ptr.wrapping_sub(3);
                mem[ptr + 3] += mem[ptr];
                mem[ptr + 4] += mem[ptr];
                mem[ptr] = 0;
                ptr = ptr.wrapping_sub(15);
            }
            ptr = ptr.wrapping_add(10);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(9);
                mem[ptr - 4] += mem[ptr];
                mem[ptr] = 0;
                ptr = ptr.wrapping_add(1);
            }
            ptr = ptr.wrapping_sub(10);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(8);
                mem[ptr - 1] -= mem[ptr];
                mem[ptr] = 0;
                ptr = ptr.wrapping_sub(18);
            }
            ptr = ptr.wrapping_add(10);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(7);
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                    ptr = ptr.wrapping_add(1);
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr = ptr.wrapping_sub(1);
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr = ptr.wrapping_add(1);
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr = ptr.wrapping_sub(1);
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_sub(1);
                            ptr = ptr.wrapping_add(1);
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            ptr = ptr.wrapping_sub(1);
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr = ptr.wrapping_add(1);
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                ptr = ptr.wrapping_sub(1);
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                    ptr = ptr.wrapping_add(1);
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    ptr = ptr.wrapping_sub(1);
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr = ptr.wrapping_add(1);
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        ptr = ptr.wrapping_sub(1);
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                            ptr = ptr.wrapping_add(1);
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            ptr = ptr.wrapping_sub(1);
                                            while mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr = ptr.wrapping_add(1);
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                ptr = ptr.wrapping_sub(1);
                                                while mem[ptr] != 0 {
                                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                                    ptr = ptr.wrapping_add(1);
                                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                                    ptr = ptr.wrapping_sub(1);
                                                    while mem[ptr] != 0 {
                                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                                        ptr = ptr.wrapping_add(1);
                                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                                        ptr = ptr.wrapping_sub(1);
                                                        while mem[ptr] != 0 {
                                                            mem[ptr] = mem[ptr].wrapping_add(10);
                                                            while mem[ptr] != 0 {
                                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                                ptr = ptr.wrapping_add(1);
                                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                                ptr = ptr.wrapping_sub(1);
                                                            }
                                                            ptr = ptr.wrapping_add(10);
                                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                                            ptr = ptr.wrapping_sub(10);
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
                ptr = ptr.wrapping_add(3);
            }
            ptr = ptr.wrapping_add(7);
            mem[ptr] = mem[ptr].wrapping_add(1);
            while mem[ptr] != 0 {
                mem[ptr] = 0;
                ptr = ptr.wrapping_sub(17);
                while mem[ptr] != 0 {
                    ptr = ptr.wrapping_add(4);
                    mem[ptr] = 0;
                    ptr = ptr.wrapping_add(4);
                    mem[ptr - 4] += mem[ptr];
                    mem[ptr] = 0;
                    ptr = ptr.wrapping_sub(2);
                    mem[ptr + 2] += mem[ptr];
                    mem[ptr] = 0;
                    ptr = ptr.wrapping_sub(16);
                }
                ptr = ptr.wrapping_add(10);
                while mem[ptr] != 0 {
                    ptr = ptr.wrapping_add(8);
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr = ptr.wrapping_add(1);
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr = ptr.wrapping_sub(3);
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr = ptr.wrapping_add(2);
                    }
                    ptr = ptr.wrapping_add(2);
                }
                ptr = ptr.wrapping_sub(10);
                while mem[ptr] != 0 {
                    ptr = ptr.wrapping_add(3);
                    mem[ptr + 6] += mem[ptr];
                    mem[ptr] = 0;
                    ptr = ptr.wrapping_sub(13);
                }
                ptr = ptr.wrapping_add(10);
                while mem[ptr] != 0 {
                    ptr = ptr.wrapping_add(9);
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr = ptr.wrapping_sub(6);
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr = ptr.wrapping_add(6);
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_sub(1);
                            ptr = ptr.wrapping_sub(6);
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            ptr = ptr.wrapping_add(6);
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr = ptr.wrapping_sub(6);
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                ptr = ptr.wrapping_add(6);
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                    ptr = ptr.wrapping_sub(6);
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    ptr = ptr.wrapping_add(6);
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr = ptr.wrapping_sub(6);
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        ptr = ptr.wrapping_add(6);
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                            ptr = ptr.wrapping_sub(6);
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            ptr = ptr.wrapping_add(6);
                                            while mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr = ptr.wrapping_sub(6);
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                ptr = ptr.wrapping_add(6);
                                                while mem[ptr] != 0 {
                                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                                    ptr = ptr.wrapping_sub(6);
                                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                                    ptr = ptr.wrapping_add(6);
                                                    while mem[ptr] != 0 {
                                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                                        ptr = ptr.wrapping_sub(6);
                                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                                        ptr = ptr.wrapping_add(6);
                                                        while mem[ptr] != 0 {
                                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                                            ptr = ptr.wrapping_sub(6);
                                                            mem[ptr] = mem[ptr].wrapping_sub(9);
                                                            ptr = ptr.wrapping_add(16);
                                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                                            ptr = ptr.wrapping_sub(10);
                                                            mem[ptr - 6] += mem[ptr];
                                                            mem[ptr] = 0;
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
                    ptr = ptr.wrapping_add(1);
                }
                ptr = ptr.wrapping_add(7);
            }
            ptr = ptr.wrapping_sub(17);
            while mem[ptr] != 0 {
                ptr -= 10;
            }
            ptr = ptr.wrapping_add(10);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(8);
                mem[ptr] = 0;
                ptr = ptr.wrapping_sub(2);
                mem[ptr + 1] += mem[ptr];
                mem[ptr] = 0;
                ptr = ptr.wrapping_sub(1);
                mem[ptr + 3] += mem[ptr];
                mem[ptr] = 0;
                ptr = ptr.wrapping_add(5);
            }
            ptr = ptr.wrapping_sub(10);
            while mem[ptr] != 0 {
                mem[ptr] = mem[ptr].wrapping_add(1);
                ptr = ptr.wrapping_add(7);
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                    ptr = ptr.wrapping_sub(7);
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr = ptr.wrapping_add(7);
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr = ptr.wrapping_sub(7);
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr = ptr.wrapping_add(6);
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr = ptr.wrapping_add(1);
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_sub(1);
                            ptr = ptr.wrapping_sub(7);
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            ptr = ptr.wrapping_add(7);
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr = ptr.wrapping_sub(7);
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr = ptr.wrapping_add(6);
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                ptr = ptr.wrapping_add(1);
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                    ptr = ptr.wrapping_sub(7);
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    ptr = ptr.wrapping_add(7);
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr = ptr.wrapping_sub(7);
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr = ptr.wrapping_add(6);
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        ptr = ptr.wrapping_add(1);
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                            ptr = ptr.wrapping_sub(7);
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            ptr = ptr.wrapping_add(7);
                                            while mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr = ptr.wrapping_sub(7);
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr = ptr.wrapping_add(6);
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                ptr = ptr.wrapping_add(1);
                                                mem[ptr - 7] += mem[ptr];
                                                mem[ptr] = 0;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                ptr = ptr.wrapping_sub(7);
                mem[ptr + 7] += mem[ptr];
                mem[ptr] = 0;
                mem[ptr] = mem[ptr].wrapping_sub(1);
                ptr = ptr.wrapping_sub(10);
            }
            ptr = ptr.wrapping_add(7);
            mem[ptr - 11] += mem[ptr];
            mem[ptr] = 0;
            ptr = ptr.wrapping_add(3);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(7);
                mem[ptr - 11] += mem[ptr].wrapping_mul(5);
                mem[ptr] = 0;
                ptr = ptr.wrapping_add(3);
            }
            ptr = ptr.wrapping_sub(10);
            while mem[ptr] != 0 {
                mem[ptr] = mem[ptr].wrapping_add(1);
                ptr = ptr.wrapping_add(8);
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                    ptr = ptr.wrapping_sub(8);
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr = ptr.wrapping_add(8);
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr = ptr.wrapping_sub(8);
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr = ptr.wrapping_add(5);
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr = ptr.wrapping_add(3);
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_sub(1);
                            ptr = ptr.wrapping_sub(8);
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            ptr = ptr.wrapping_add(8);
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr = ptr.wrapping_sub(8);
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr = ptr.wrapping_add(5);
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                ptr = ptr.wrapping_add(3);
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                    ptr = ptr.wrapping_sub(8);
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    ptr = ptr.wrapping_add(8);
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr = ptr.wrapping_sub(8);
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr = ptr.wrapping_add(5);
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        ptr = ptr.wrapping_add(3);
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                            ptr = ptr.wrapping_sub(8);
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            ptr = ptr.wrapping_add(8);
                                            while mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr = ptr.wrapping_sub(8);
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr = ptr.wrapping_add(5);
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                ptr = ptr.wrapping_add(3);
                                                mem[ptr - 8] += mem[ptr];
                                                mem[ptr] = 0;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                ptr = ptr.wrapping_sub(8);
                mem[ptr + 8] += mem[ptr];
                mem[ptr] = 0;
                mem[ptr] = mem[ptr].wrapping_sub(1);
                ptr = ptr.wrapping_sub(10);
            }
            ptr = ptr.wrapping_add(8);
            mem[ptr - 13] += mem[ptr];
            mem[ptr] = 0;
            ptr = ptr.wrapping_add(2);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(8);
                mem[ptr - 13] += mem[ptr].wrapping_mul(5);
                mem[ptr] = 0;
                ptr = ptr.wrapping_add(2);
            }
            ptr = ptr.wrapping_sub(10);
            while mem[ptr] != 0 {
                ptr -= 10;
            }
            ptr = ptr.wrapping_add(16);
        }
        ptr = ptr.wrapping_sub(6);
        while mem[ptr] != 0 {
            ptr = ptr.wrapping_add(3);
            mem[ptr + 4] += mem[ptr];
            mem[ptr + 5] += mem[ptr];
            mem[ptr] = 0;
            ptr = ptr.wrapping_add(7);
        }
        ptr = ptr.wrapping_sub(10);
        while mem[ptr] != 0 {
            ptr = ptr.wrapping_add(7);
            mem[ptr - 4] += mem[ptr];
            mem[ptr] = 0;
            ptr = ptr.wrapping_sub(5);
            mem[ptr + 5] += mem[ptr];
            mem[ptr + 7] += mem[ptr];
            mem[ptr] = 0;
            ptr = ptr.wrapping_sub(12);
        }
        ptr = ptr.wrapping_add(10);
        while mem[ptr] != 0 {
            ptr = ptr.wrapping_add(7);
            mem[ptr - 5] += mem[ptr];
            mem[ptr] = 0;
            ptr = ptr.wrapping_add(3);
        }
        ptr = ptr.wrapping_sub(10);
        while mem[ptr] != 0 {
            ptr = ptr.wrapping_add(9);
            mem[ptr - 1] -= mem[ptr];
            mem[ptr] = 0;
            ptr = ptr.wrapping_sub(1);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_sub(8);
                while mem[ptr] != 0 {
                    ptr = ptr.wrapping_sub(2);
                    mem[ptr] = 0;
                    ptr = ptr.wrapping_add(10);
                    mem[ptr - 10] += mem[ptr];
                    mem[ptr] = 0;
                    ptr = ptr.wrapping_sub(18);
                }
                ptr = ptr.wrapping_add(18);
            }
            ptr = ptr.wrapping_sub(18);
        }
        ptr = ptr.wrapping_add(8);
        while mem[ptr] != 0 {
            ptr = ptr.wrapping_add(1);
            mem[ptr] = mem[ptr].wrapping_sub(1);
            ptr = ptr.wrapping_sub(1);
            while mem[ptr] != 0 {
                mem[ptr] = mem[ptr].wrapping_add(1);
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            while mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                while mem[ptr] != 0 {
                                                    mem[ptr] = 0;
                                                    ptr = ptr.wrapping_add(1);
                                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                                    ptr = ptr.wrapping_sub(1);
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
        ptr = ptr.wrapping_add(1);
        mem[ptr] = mem[ptr].wrapping_add(1);
        while mem[ptr] != 0 {
            mem[ptr] = 0;
            ptr = ptr.wrapping_sub(1);
            mem[ptr] = 0;
            mem[ptr] = mem[ptr].wrapping_add(1);
            ptr = ptr.wrapping_add(4);
            mem[ptr] = mem[ptr].wrapping_add(1);
            ptr = ptr.wrapping_add(8);
            while mem[ptr] != 0 {
                ptr += 10;
            }
            ptr = ptr.wrapping_sub(10);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_sub(6);
                while mem[ptr] != 0 {
                    ptr = ptr.wrapping_sub(4);
                    while mem[ptr] != 0 {
                        ptr -= 10;
                    }
                    ptr = ptr.wrapping_add(4);
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr = ptr.wrapping_sub(10);
                }
                ptr = ptr.wrapping_sub(4);
            }
            ptr = ptr.wrapping_add(20);
            while mem[ptr] != 0 {
                ptr += 10;
            }
            ptr = ptr.wrapping_sub(10);
            while mem[ptr] != 0 {
                ptr -= 10;
            }
            ptr = ptr.wrapping_add(4);
            mem[ptr] = mem[ptr].wrapping_sub(1);
            while mem[ptr] != 0 {
                mem[ptr] = 0;
                ptr = ptr.wrapping_add(8);
                mem[ptr] = mem[ptr].wrapping_sub(1);
                ptr = ptr.wrapping_sub(2);
                while mem[ptr] != 0 {
                    ptr = ptr.wrapping_add(1);
                    mem[ptr] = 0;
                    ptr = ptr.wrapping_add(2);
                    mem[ptr - 2] += mem[ptr];
                    mem[ptr] = 0;
                    ptr = ptr.wrapping_add(7);
                }
                ptr = ptr.wrapping_sub(10);
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr = ptr.wrapping_add(2);
                    while mem[ptr] != 0 {
                        ptr = ptr.wrapping_add(8);
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr = ptr.wrapping_add(2);
                    }
                    ptr = ptr.wrapping_sub(2);
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                    ptr = ptr.wrapping_sub(10);
                }
                mem[ptr] = mem[ptr].wrapping_sub(1);
                while mem[ptr] != 0 {
                    ptr = ptr.wrapping_add(2);
                    mem[ptr] = mem[ptr].wrapping_add(48);
                    writer.write_all(&mem[ptr..ptr + 1]).unwrap();
                    mem[ptr] = mem[ptr].wrapping_sub(48);
                    ptr = ptr.wrapping_sub(12);
                }
                mem[ptr] = mem[ptr].wrapping_add(32);
                writer.write_all(&mem[ptr..ptr + 1]).unwrap();
                mem[ptr] = 0;
                ptr = ptr.wrapping_add(4);
            }
            ptr = ptr.wrapping_add(6);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(2);
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                    ptr = ptr.wrapping_add(5);
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr = ptr.wrapping_sub(5);
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr = ptr.wrapping_add(5);
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr = ptr.wrapping_sub(5);
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_sub(1);
                            ptr = ptr.wrapping_add(5);
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            ptr = ptr.wrapping_sub(5);
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr = ptr.wrapping_add(5);
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                ptr = ptr.wrapping_sub(5);
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                    ptr = ptr.wrapping_add(5);
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    ptr = ptr.wrapping_sub(5);
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr = ptr.wrapping_add(5);
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        ptr = ptr.wrapping_sub(5);
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                            ptr = ptr.wrapping_add(5);
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            ptr = ptr.wrapping_sub(5);
                                            while mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr = ptr.wrapping_add(5);
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                ptr = ptr.wrapping_sub(5);
                                                while mem[ptr] != 0 {
                                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                                    ptr = ptr.wrapping_add(5);
                                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                                    ptr = ptr.wrapping_sub(5);
                                                    while mem[ptr] != 0 {
                                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                                        ptr = ptr.wrapping_add(5);
                                                        mem[ptr] = mem[ptr].wrapping_sub(9);
                                                        ptr = ptr.wrapping_add(5);
                                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                                        ptr = ptr.wrapping_sub(10);
                                                        mem[ptr + 5] += mem[ptr];
                                                        mem[ptr] = 0;
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
                ptr = ptr.wrapping_add(8);
            }
            ptr = ptr.wrapping_sub(10);
            while mem[ptr] != 0 {
                ptr = ptr.wrapping_add(7);
                mem[ptr - 5] += mem[ptr];
                mem[ptr] = 0;
                ptr = ptr.wrapping_sub(17);
            }
            ptr = ptr.wrapping_add(9);
        }
        ptr = ptr.wrapping_sub(1);
    }
    ptr = ptr.wrapping_add(2);
    while mem[ptr] != 0 {
        ptr += 10;
    }
    ptr = ptr.wrapping_sub(10);
    while mem[ptr] != 0 {
        mem[ptr] = mem[ptr].wrapping_add(1);
        ptr = ptr.wrapping_add(1);
        while mem[ptr] != 0 {
            ptr = ptr.wrapping_add(9);
            mem[ptr] = mem[ptr].wrapping_add(1);
            ptr = ptr.wrapping_add(1);
        }
        ptr = ptr.wrapping_sub(1);
        mem[ptr] = mem[ptr].wrapping_sub(1);
        ptr = ptr.wrapping_sub(10);
    }
    mem[ptr] = mem[ptr].wrapping_sub(1);
    while mem[ptr] != 0 {
        ptr = ptr.wrapping_add(1);
        mem[ptr] = mem[ptr].wrapping_add(48);
        writer.write_all(&mem[ptr..ptr + 1]).unwrap();
        ptr = ptr.wrapping_sub(11);
    }
    mem[ptr] = mem[ptr].wrapping_add(10);
    writer.write_all(&mem[ptr..ptr + 1]).unwrap();
}
