pub fn bf_main<R: std::io::Read, W: std::io::Write>(
    mut reader: R,
    mut writer: W,
) {
    let mut mem:Vec<u8> = vec![0;30000];
    let mut ptr:usize = 0;
    while mem[ptr] != 0 {
    }
    ptr += 30;
    mem[ptr] = mem[ptr].wrapping_sub(1);
    ptr -= 9;
    mem[ptr] = mem[ptr].wrapping_add(1);
    while mem[ptr] != 0 {
        mem[ptr] = mem[ptr].wrapping_sub(1);
        while mem[ptr] != 0 {
            ptr += 10;
        }
        ptr -= 10;
        while mem[ptr] != 0 {
            mem[ptr + 10] += mem[ptr];
            mem[ptr] = 0;
            ptr -= 10;
        }
        ptr += 10;
        reader.read_exact(&mut mem[ptr..ptr + 1]).unwrap();
        mem[ptr] = mem[ptr].wrapping_sub(10);
    }
    ptr += 10;
    while mem[ptr] != 0 {
        mem[ptr] = mem[ptr].wrapping_sub(37);
        ptr += 9;
        mem[ptr] = mem[ptr].wrapping_sub(1);
        ptr += 1;
    }
    ptr -= 1;
    while mem[ptr] != 0 {
        mem[ptr] = mem[ptr].wrapping_add(1);
        ptr += 1;
        while mem[ptr] != 0 {
            ptr += 9;
            mem[ptr] = mem[ptr].wrapping_add(1);
            ptr += 1;
        }
        ptr -= 1;
        mem[ptr] = mem[ptr].wrapping_sub(1);
        ptr -= 10;
    }
    mem[ptr] = mem[ptr].wrapping_sub(1);
    while mem[ptr] != 0 {
        ptr += 1;
        mem[ptr] = mem[ptr].wrapping_add(48);
        writer.write_all(&mem[ptr..ptr + 1]).unwrap();
        mem[ptr] = mem[ptr].wrapping_sub(48);
        ptr -= 11;
    }
    mem[ptr] = mem[ptr].wrapping_add(58);
    writer.write_all(&mem[ptr..ptr + 1]).unwrap();
    mem[ptr] = mem[ptr].wrapping_sub(26);
    writer.write_all(&mem[ptr..ptr + 1]).unwrap();
    mem[ptr] = 0;
    ptr += 12;
    mem[ptr] = mem[ptr].wrapping_add(2);
    ptr -= 4;
    mem[ptr] = mem[ptr].wrapping_add(1);
    while mem[ptr] != 0 {
        mem[ptr] = 0;
        ptr += 2;
        while mem[ptr] != 0 {
            ptr += 4;
            mem[ptr] = 0;
            ptr += 1;
            mem[ptr] = 0;
            ptr += 1;
            mem[ptr] = 0;
            ptr += 1;
            mem[ptr] = 0;
            ptr += 1;
            mem[ptr] = 0;
            ptr += 1;
            mem[ptr] = 0;
            ptr -= 7;
            mem[ptr + 3] += mem[ptr];
            mem[ptr + 4] += mem[ptr];
            mem[ptr] = 0;
            ptr += 8;
        }
        ptr -= 10;
        while mem[ptr] != 0 {
            ptr += 6;
            mem[ptr - 4] += mem[ptr];
            mem[ptr] = 0;
            ptr -= 16;
        }
        ptr += 10;
        while mem[ptr] != 0 {
            ptr += 1;
            mem[ptr + 3] += mem[ptr];
            mem[ptr + 5] += mem[ptr];
            mem[ptr] = 0;
            ptr += 9;
        }
        ptr -= 10;
        while mem[ptr] != 0 {
            ptr += 6;
            mem[ptr - 5] += mem[ptr];
            mem[ptr] = 0;
            ptr -= 16;
        }
        ptr += 10;
        while mem[ptr] != 0 {
            ptr += 3;
            mem[ptr] = 0;
            ptr += 3;
            mem[ptr] = 0;
            ptr += 1;
            mem[ptr] = 0;
            ptr += 3;
        }
        ptr -= 10;
        while mem[ptr] != 0 {
            ptr -= 10;
        }
        ptr += 9;
        mem[ptr] = 0;
        ptr += 7;
        mem[ptr] = mem[ptr].wrapping_add(1);
        ptr -= 8;
        mem[ptr] = 1;
        while mem[ptr] != 0 {
            mem[ptr] = mem[ptr].wrapping_sub(1);
            ptr += 2;
            while mem[ptr] != 0 {
                ptr += 6;
                mem[ptr + 1] += mem[ptr].wrapping_mul(2);
                mem[ptr] = 0;
                ptr += 4;
            }
            ptr -= 10;
            while mem[ptr] != 0 {
                ptr += 8;
                mem[ptr] = 0;
                ptr += 1;
                mem[ptr] = 0;
                ptr -= 4;
                mem[ptr + 3] += mem[ptr].wrapping_mul(2);
                mem[ptr] = 0;
                ptr -= 15;
            }
            ptr += 10;
            while mem[ptr] != 0 {
                ptr += 8;
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                    ptr += 1;
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr -= 1;
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr += 1;
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr -= 1;
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_sub(1);
                            ptr += 1;
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            ptr -= 1;
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr += 1;
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                ptr -= 1;
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                    ptr += 1;
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    ptr -= 1;
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr += 1;
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        ptr -= 1;
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                            ptr += 1;
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            ptr -= 1;
                                            while mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr += 1;
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                ptr -= 1;
                                                while mem[ptr] != 0 {
                                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                                    ptr += 1;
                                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                                    ptr -= 1;
                                                    if mem[ptr] != 0 {
                                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                                        ptr += 1;
                                                        mem[ptr] = mem[ptr].wrapping_sub(9);
                                                        ptr += 9;
                                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                                        ptr -= 10;
                                                        mem[ptr + 1] += mem[ptr];
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
                ptr += 2;
            }
            ptr -= 10;
            while mem[ptr] != 0 {
                ptr += 9;
                mem[ptr - 1] += mem[ptr];
                mem[ptr - 4] += mem[ptr];
                mem[ptr] = 0;
                ptr -= 19;
            }
            ptr += 10;
            while mem[ptr] != 0 {
                ptr += 7;
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                    ptr -= 1;
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr += 1;
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr -= 1;
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr += 1;
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_sub(1);
                            ptr -= 1;
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            ptr += 1;
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr -= 1;
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                ptr += 1;
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                    ptr -= 1;
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    ptr += 1;
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr -= 1;
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        ptr += 1;
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                            ptr -= 1;
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            ptr += 1;
                                            while mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr -= 1;
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                ptr += 1;
                                                while mem[ptr] != 0 {
                                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                                    ptr -= 1;
                                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                                    ptr += 1;
                                                    if mem[ptr] != 0 {
                                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                                        ptr -= 1;
                                                        mem[ptr] = mem[ptr].wrapping_sub(9);
                                                        ptr += 11;
                                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                                        ptr -= 10;
                                                        mem[ptr - 1] += mem[ptr];
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
                ptr += 3;
            }
            ptr -= 10;
            while mem[ptr] != 0 {
                ptr += 4;
                mem[ptr + 3] += mem[ptr];
                mem[ptr + 5] += mem[ptr];
                mem[ptr] = 0;
                ptr -= 14;
            }
            ptr += 10;
            while mem[ptr] != 0 {
                ptr += 7;
                mem[ptr - 3] += mem[ptr];
                mem[ptr] = 0;
                ptr += 3;
            }
            ptr -= 10;
            while mem[ptr] != 0 {
                ptr += 8;
                mem[ptr + 1] -= mem[ptr];
                mem[ptr] = 0;
                ptr += 1;
                while mem[ptr] != 0 {
                    ptr -= 9;
                    while mem[ptr] != 0 {
                        ptr -= 1;
                        mem[ptr] = 0;
                        ptr += 10;
                        mem[ptr - 10] += mem[ptr];
                        mem[ptr] = 0;
                        ptr -= 19;
                    }
                    ptr += 19;
                }
                ptr -= 19;
            }
            ptr += 9;
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
                                                        ptr -= 1;
                                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                                        ptr += 1;
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
            ptr -= 1;
        }
        ptr += 8;
        while mem[ptr] != 0 {
            ptr -= 6;
            while mem[ptr] != 0 {
                ptr += 8;
                mem[ptr] = 0;
                ptr += 1;
                mem[ptr] = 0;
                ptr -= 5;
                mem[ptr + 3] += mem[ptr];
                mem[ptr + 4] += mem[ptr];
                mem[ptr] = 0;
                ptr += 6;
            }
            ptr -= 10;
            while mem[ptr] != 0 {
                ptr += 8;
                mem[ptr - 4] += mem[ptr];
                mem[ptr] = 0;
                ptr -= 3;
                mem[ptr + 3] += mem[ptr];
                mem[ptr + 4] += mem[ptr];
                mem[ptr] = 0;
                ptr -= 15;
            }
            ptr += 10;
            while mem[ptr] != 0 {
                ptr += 9;
                mem[ptr - 4] += mem[ptr];
                mem[ptr] = 0;
                ptr += 1;
            }
            ptr -= 10;
            while mem[ptr] != 0 {
                ptr += 8;
                mem[ptr - 1] -= mem[ptr];
                mem[ptr] = 0;
                ptr -= 18;
            }
            ptr += 10;
            while mem[ptr] != 0 {
                ptr += 7;
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                    ptr += 1;
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr -= 1;
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr += 1;
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr -= 1;
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_sub(1);
                            ptr += 1;
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            ptr -= 1;
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr += 1;
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                ptr -= 1;
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                    ptr += 1;
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    ptr -= 1;
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr += 1;
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        ptr -= 1;
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                            ptr += 1;
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            ptr -= 1;
                                            while mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr += 1;
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                ptr -= 1;
                                                while mem[ptr] != 0 {
                                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                                    ptr += 1;
                                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                                    ptr -= 1;
                                                    while mem[ptr] != 0 {
                                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                                        ptr += 1;
                                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                                        ptr -= 1;
                                                        while mem[ptr] != 0 {
                                                            mem[ptr] = mem[ptr].wrapping_add(10);
                                                            while mem[ptr] != 0 {
                                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                                ptr += 1;
                                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                                ptr -= 1;
                                                            }
                                                            ptr += 10;
                                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                                            ptr -= 10;
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
                ptr += 3;
            }
            ptr += 7;
            mem[ptr] = mem[ptr].wrapping_add(1);
            while mem[ptr] != 0 {
                mem[ptr] = 0;
                ptr -= 17;
                while mem[ptr] != 0 {
                    ptr += 4;
                    mem[ptr] = 0;
                    ptr += 4;
                    mem[ptr - 4] += mem[ptr];
                    mem[ptr] = 0;
                    ptr -= 2;
                    mem[ptr + 2] += mem[ptr];
                    mem[ptr] = 0;
                    ptr -= 16;
                }
                ptr += 10;
                while mem[ptr] != 0 {
                    ptr += 8;
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr += 1;
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr -= 3;
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr += 2;
                    }
                    ptr += 2;
                }
                ptr -= 10;
                while mem[ptr] != 0 {
                    ptr += 3;
                    mem[ptr + 6] += mem[ptr];
                    mem[ptr] = 0;
                    ptr -= 13;
                }
                ptr += 10;
                while mem[ptr] != 0 {
                    ptr += 9;
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr -= 6;
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr += 6;
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_sub(1);
                            ptr -= 6;
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            ptr += 6;
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr -= 6;
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                ptr += 6;
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                    ptr -= 6;
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    ptr += 6;
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr -= 6;
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        ptr += 6;
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                            ptr -= 6;
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            ptr += 6;
                                            while mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr -= 6;
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                ptr += 6;
                                                while mem[ptr] != 0 {
                                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                                    ptr -= 6;
                                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                                    ptr += 6;
                                                    while mem[ptr] != 0 {
                                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                                        ptr -= 6;
                                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                                        ptr += 6;
                                                        if mem[ptr] != 0 {
                                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                                            ptr -= 6;
                                                            mem[ptr] = mem[ptr].wrapping_sub(9);
                                                            ptr += 16;
                                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                                            ptr -= 10;
                                                            mem[ptr - 6] += mem[ptr];
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
                    ptr += 1;
                }
                ptr += 7;
            }
            ptr -= 17;
            while mem[ptr] != 0 {
                ptr -= 10;
            }
            ptr += 10;
            while mem[ptr] != 0 {
                ptr += 8;
                mem[ptr] = 0;
                ptr -= 2;
                mem[ptr + 1] += mem[ptr];
                mem[ptr] = 0;
                ptr -= 1;
                mem[ptr + 3] += mem[ptr];
                mem[ptr] = 0;
                ptr += 5;
            }
            ptr -= 10;
            while mem[ptr] != 0 {
                mem[ptr] = mem[ptr].wrapping_add(1);
                ptr += 7;
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                    ptr -= 7;
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr += 7;
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr -= 7;
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr += 6;
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr += 1;
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_sub(1);
                            ptr -= 7;
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            ptr += 7;
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr -= 7;
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr += 6;
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                ptr += 1;
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                    ptr -= 7;
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    ptr += 7;
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr -= 7;
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr += 6;
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        ptr += 1;
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                            ptr -= 7;
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            ptr += 7;
                                            if mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr -= 7;
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr += 6;
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                ptr += 1;
                                                mem[ptr - 7] += mem[ptr];
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                ptr -= 7;
                mem[ptr + 7] += mem[ptr];
                mem[ptr] = 255;
                ptr -= 10;
            }
            ptr += 7;
            mem[ptr - 11] += mem[ptr];
            mem[ptr] = 0;
            ptr += 3;
            while mem[ptr] != 0 {
                ptr += 7;
                mem[ptr - 11] += mem[ptr].wrapping_mul(5);
                mem[ptr] = 0;
                ptr += 3;
            }
            ptr -= 10;
            while mem[ptr] != 0 {
                mem[ptr] = mem[ptr].wrapping_add(1);
                ptr += 8;
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                    ptr -= 8;
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr += 8;
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr -= 8;
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr += 5;
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr += 3;
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_sub(1);
                            ptr -= 8;
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            ptr += 8;
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr -= 8;
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr += 5;
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                ptr += 3;
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                    ptr -= 8;
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    ptr += 8;
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr -= 8;
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr += 5;
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        ptr += 3;
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                            ptr -= 8;
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            ptr += 8;
                                            if mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr -= 8;
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr += 5;
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                ptr += 3;
                                                mem[ptr - 8] += mem[ptr];
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                ptr -= 8;
                mem[ptr + 8] += mem[ptr];
                mem[ptr] = 255;
                ptr -= 10;
            }
            ptr += 8;
            mem[ptr - 13] += mem[ptr];
            mem[ptr] = 0;
            ptr += 2;
            while mem[ptr] != 0 {
                ptr += 8;
                mem[ptr - 13] += mem[ptr].wrapping_mul(5);
                mem[ptr] = 0;
                ptr += 2;
            }
            ptr -= 10;
            while mem[ptr] != 0 {
                ptr -= 10;
            }
            ptr += 16;
        }
        ptr -= 6;
        while mem[ptr] != 0 {
            ptr += 3;
            mem[ptr + 4] += mem[ptr];
            mem[ptr + 5] += mem[ptr];
            mem[ptr] = 0;
            ptr += 7;
        }
        ptr -= 10;
        while mem[ptr] != 0 {
            ptr += 7;
            mem[ptr - 4] += mem[ptr];
            mem[ptr] = 0;
            ptr -= 5;
            mem[ptr + 5] += mem[ptr];
            mem[ptr + 7] += mem[ptr];
            mem[ptr] = 0;
            ptr -= 12;
        }
        ptr += 10;
        while mem[ptr] != 0 {
            ptr += 7;
            mem[ptr - 5] += mem[ptr];
            mem[ptr] = 0;
            ptr += 3;
        }
        ptr -= 10;
        while mem[ptr] != 0 {
            ptr += 9;
            mem[ptr - 1] -= mem[ptr];
            mem[ptr] = 0;
            ptr -= 1;
            while mem[ptr] != 0 {
                ptr -= 8;
                while mem[ptr] != 0 {
                    ptr -= 2;
                    mem[ptr] = 0;
                    ptr += 10;
                    mem[ptr - 10] += mem[ptr];
                    mem[ptr] = 0;
                    ptr -= 18;
                }
                ptr += 18;
            }
            ptr -= 18;
        }
        ptr += 8;
        while mem[ptr] != 0 {
            ptr += 1;
            mem[ptr] = mem[ptr].wrapping_sub(1);
            ptr -= 1;
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
                                                    ptr += 1;
                                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                                    ptr -= 1;
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
        ptr += 1;
        mem[ptr] = mem[ptr].wrapping_add(1);
        while mem[ptr] != 0 {
            mem[ptr] = 0;
            ptr -= 1;
            mem[ptr] = 1;
            ptr += 4;
            mem[ptr] = mem[ptr].wrapping_add(1);
            ptr += 8;
            while mem[ptr] != 0 {
                ptr += 10;
            }
            ptr -= 10;
            while mem[ptr] != 0 {
                ptr -= 6;
                while mem[ptr] != 0 {
                    ptr -= 4;
                    while mem[ptr] != 0 {
                        ptr -= 10;
                    }
                    ptr += 4;
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr -= 10;
                }
                ptr -= 4;
            }
            ptr += 20;
            while mem[ptr] != 0 {
                ptr += 10;
            }
            ptr -= 10;
            while mem[ptr] != 0 {
                ptr -= 10;
            }
            ptr += 4;
            mem[ptr] = mem[ptr].wrapping_sub(1);
            while mem[ptr] != 0 {
                mem[ptr] = 0;
                ptr += 8;
                mem[ptr] = mem[ptr].wrapping_sub(1);
                ptr -= 2;
                while mem[ptr] != 0 {
                    ptr += 1;
                    mem[ptr] = 0;
                    ptr += 2;
                    mem[ptr - 2] += mem[ptr];
                    mem[ptr] = 0;
                    ptr += 7;
                }
                ptr -= 10;
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr += 2;
                    while mem[ptr] != 0 {
                        ptr += 8;
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr += 2;
                    }
                    ptr -= 2;
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                    ptr -= 10;
                }
                mem[ptr] = mem[ptr].wrapping_sub(1);
                while mem[ptr] != 0 {
                    ptr += 2;
                    mem[ptr] = mem[ptr].wrapping_add(48);
                    writer.write_all(&mem[ptr..ptr + 1]).unwrap();
                    mem[ptr] = mem[ptr].wrapping_sub(48);
                    ptr -= 12;
                }
                mem[ptr] = mem[ptr].wrapping_add(32);
                writer.write_all(&mem[ptr..ptr + 1]).unwrap();
                mem[ptr] = 0;
                ptr += 4;
            }
            ptr += 6;
            while mem[ptr] != 0 {
                ptr += 2;
                while mem[ptr] != 0 {
                    mem[ptr] = mem[ptr].wrapping_sub(1);
                    ptr += 5;
                    mem[ptr] = mem[ptr].wrapping_add(1);
                    ptr -= 5;
                    while mem[ptr] != 0 {
                        mem[ptr] = mem[ptr].wrapping_sub(1);
                        ptr += 5;
                        mem[ptr] = mem[ptr].wrapping_add(1);
                        ptr -= 5;
                        while mem[ptr] != 0 {
                            mem[ptr] = mem[ptr].wrapping_sub(1);
                            ptr += 5;
                            mem[ptr] = mem[ptr].wrapping_add(1);
                            ptr -= 5;
                            while mem[ptr] != 0 {
                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                ptr += 5;
                                mem[ptr] = mem[ptr].wrapping_add(1);
                                ptr -= 5;
                                while mem[ptr] != 0 {
                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                    ptr += 5;
                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                    ptr -= 5;
                                    while mem[ptr] != 0 {
                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                        ptr += 5;
                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                        ptr -= 5;
                                        while mem[ptr] != 0 {
                                            mem[ptr] = mem[ptr].wrapping_sub(1);
                                            ptr += 5;
                                            mem[ptr] = mem[ptr].wrapping_add(1);
                                            ptr -= 5;
                                            while mem[ptr] != 0 {
                                                mem[ptr] = mem[ptr].wrapping_sub(1);
                                                ptr += 5;
                                                mem[ptr] = mem[ptr].wrapping_add(1);
                                                ptr -= 5;
                                                while mem[ptr] != 0 {
                                                    mem[ptr] = mem[ptr].wrapping_sub(1);
                                                    ptr += 5;
                                                    mem[ptr] = mem[ptr].wrapping_add(1);
                                                    ptr -= 5;
                                                    if mem[ptr] != 0 {
                                                        mem[ptr] = mem[ptr].wrapping_sub(1);
                                                        ptr += 5;
                                                        mem[ptr] = mem[ptr].wrapping_sub(9);
                                                        ptr += 5;
                                                        mem[ptr] = mem[ptr].wrapping_add(1);
                                                        ptr -= 10;
                                                        mem[ptr + 5] += mem[ptr];
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
                ptr += 8;
            }
            ptr -= 10;
            while mem[ptr] != 0 {
                ptr += 7;
                mem[ptr - 5] += mem[ptr];
                mem[ptr] = 0;
                ptr -= 17;
            }
            ptr += 9;
        }
        ptr -= 1;
    }
    ptr += 2;
    while mem[ptr] != 0 {
        ptr += 10;
    }
    ptr -= 10;
    while mem[ptr] != 0 {
        mem[ptr] = mem[ptr].wrapping_add(1);
        ptr += 1;
        while mem[ptr] != 0 {
            ptr += 9;
            mem[ptr] = mem[ptr].wrapping_add(1);
            ptr += 1;
        }
        ptr -= 1;
        mem[ptr] = mem[ptr].wrapping_sub(1);
        ptr -= 10;
    }
    mem[ptr] = mem[ptr].wrapping_sub(1);
    while mem[ptr] != 0 {
        ptr += 1;
        mem[ptr] = mem[ptr].wrapping_add(48);
        writer.write_all(&mem[ptr..ptr + 1]).unwrap();
        ptr -= 11;
    }
    mem[ptr] = mem[ptr].wrapping_add(10);
    writer.write_all(&mem[ptr..ptr + 1]).unwrap();
}