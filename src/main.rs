const GRADATION_MODE: bool = false;   // グラデーションにするか？
const SIDE_SIZE: f32       = 16.0;    // 辺の長さ
const VIEW_SIZE_RATE: f32  = 0.95;    // 画面に対する表示領域の大きさ
const COLOR_MAX: i32       = 255;     // 色の要素の最大
const FPS: u32             = 60;      // 決め打ち

//   PRESET_LIST = [
//     { favorite: false, name: "default",                                              func: -> (t, i, x, y) { sin(y/8+t)                                                     }},
//     { favorite: false, name: "for every dot return 0 or 1 to change the visibility", func: -> (t, i, x, y) { rand < 0.1                                                     }},
//     { favorite: false, name: "use a float between 0 and 1 to define the size",       func: -> (t, i, x, y) { rand                                                           }},
//     { favorite: false, name: "parameter `t` is the time in seconds",                 func: -> (t, i, x, y) { sin(t)                                                         }},
//     { favorite: false, name: "parameter `i` is the index of the dot (0..255)",       func: -> (t, i, x, y) { i / 256                                                        }},
//     { favorite: false, name: "`x` is the column index from 0 to 15",                 func: -> (t, i, x, y) { x / 16                                                         }},
//     { favorite: false, name: "`y` is the row also from 0 to 15",                     func: -> (t, i, x, y) { y / 16                                                         }},
//     { favorite: false, name: "positive numbers are white, negatives are red",        func: -> (t, i, x, y) { y - 7.5                                                        }},
//     { favorite: false, name: "use the time to animate values",                       func: -> (t, i, x, y) { y - t                                                          }},
//     { favorite: false, name: "multiply the time to change the speed",                func: -> (t, i, x, y) { y - t*4                                                        }},
//     { favorite: false, name: "create PresetInfo using different color",              func: -> (t, i, x, y) { [1, 0, -1][i%3]                                                }},
//     { favorite: true,  name: "skip `Math` to use methods",                           func: -> (t, i, x, y) { sin(t-sqrt((x-7.5)**2+(y-6)**2))                               }},
//     { favorite: false, name: "more examples",                                        func: -> (t, i, x, y) { sin(y/8 + t)                                                   }},
//     { favorite: false, name: "simple triangle",                                      func: -> (t, i, x, y) { y - x                                                          }},
//     { favorite: false, name: "quarter triangle",                                     func: -> (t, i, x, y) { (y > x) && (14-x < y)                                          }},
//     { favorite: false, name: "pattern",                                              func: -> (t, i, x, y) { i%4 - y%4                                                      }},
//     { favorite: false, name: "grid",                                                 func: -> (t, i, x, y) { (i%4)>0 && (y%4)>0                                             }},
//     { favorite: false, name: "square",                                               func: -> (t, i, x, y) { x>3 && y>3 && x<12 && y<12                                     }},
//     { favorite: false, name: "animated square",                                      func: -> (t, i, x, y) { (x>t && y>t && x<15-t && y<15-t) ? -1 : 0                      }},
//     { favorite: false, name: "mondrian squares",                                     func: -> (t, i, x, y) { (y-6) * (x-6)                                                  }},
//     { favorite: true,  name: "moving cross",                                         func: -> (t, i, x, y) { (y-4*t) * (x-2-t)                                              }},
//     { favorite: false, name: "sierpinski",                                           func: -> (t, i, x, y) { (4*t).to_i & i.to_i & x.to_i & y.to_i                          }},
//     { favorite: false, name: "binary clock",                                         func: -> (t, i, x, y) { y==8 && (t*10).to_i & (1<<x)                                   }},
//     { favorite: false, name: "random noise",                                         func: -> (t, i, x, y) { rand(-1.0..1.0)                                                }},
//     { favorite: false, name: "static smooth noise",                                  func: -> (t, i, x, y) { sin(i**2)                                                      }},
//     { favorite: true,  name: "animated smooth noise",                                func: -> (t, i, x, y) { cos(t + i + x * y)                                             }},
//     { favorite: true,  name: "waves",                                                func: -> (t, i, x, y) { sin(x/2) - sin(x-t) - y+6                                      }},
//     { favorite: true,  name: "bloop bloop bloop by @v21",                            func: -> (t, i, x, y) { (x-8)*(y-8) - sin(t)*64                                        }},
//     { favorite: true,  name: "fireworks by @p_malin and @aemkei",                    func: -> (t, i, x, y) { -0.4/(hypot(x-t%10,y-t%8)-t%2*9)                               }},
//     { favorite: true,  name: "ripples by @thespite",                                 func: -> (t, i, x, y) { sin(t-sqrt(x*x+y*y))                                           }},
//     { favorite: true,  name: "scrolling TIXY font by @atesgoral",                    func: -> (t, i, x, y) { [5463,2194,2386][y.to_i + (t*9).to_i & 7] & (1 << x-1)         }},
//     { favorite: true,  name: "3d checker board by @p_malin",                         func: -> (t, i, x, y) { y>0 && (((x-8) / y + t*5).to_i & 1 ^ (1/y*8).to_i & 1) * y / 5 }},
//     { favorite: false, name: "sticky blood by @joeytwiddle",                         func: -> (t, i, x, y) { y-t*3+9+3*cos(x*3-t)-5*sin(x*7)                                }},
//     { favorite: true,  name: "3d starfield by @p_malin",                             func: -> (t, i, x, y) { d=y*y%5.9+1;(((x+t*50/d).to_i&15).zero? ? 1/d : 0)             }},
//     { favorite: false, name: "dialogue with an alien by @chiptune",                  func: -> (t, i, x, y) { 1.0/32.0*tan(t/64.0*x*tan(i-x))                                }},
//     { favorite: true,  name: "space invader by @keithclarkcouk + @zozuar",           func: -> (t, i, x, y) { 'p}¶¼<¼¶}p'.codepoints[x] & 2**y.to_i                        }},
//     { favorite: true,  name: "hungry pac man by @p_malin and @aemkei",               func: -> (t, i, x, y) { hypot(x-=t%4*5,y-=8)<6 && (x<y || y<-x)                        }},
//     { favorite: false, name: "spectrum analyser by @joeytwiddle",                    func: -> (t, i, x, y) { x.to_i.even? && y < 9 && y > (4 + sin(8*t+x*x) + x / 4)        }},
//     { favorite: false, name: "diagonals",                                            func: -> (t, i, x, y) { y == x || ((15-x == y) ? -1 : 0 )                              }},
//     { favorite: false, name: "frame",                                                func: -> (t, i, x, y) { x==0 || x==15 || y==0 || y==15                                 }},
//     { favorite: true,  name: "drop",                                                 func: -> (t, i, x, y) { 8*t%13 - hypot(x-7.5, y-7.5)                                   }},
//     { favorite: true,  name: "rotation",                                             func: -> (t, i, x, y) { sin(2*atan((y-7.5)/(x-7.5))+5*t)                               }},
//     { favorite: true,  name: "wipe",                                                 func: -> (t, i, x, y) { (x-y) - sin(t) * 16                                            }},
//     { favorite: false, name: "soft wipe",                                            func: -> (t, i, x, y) { (x-y)/24 - sin(t)                                              }},
//     { favorite: false, name: "disco",                                                func: -> (t, i, x, y) { sin(t*5) * tan(t*7)                                            }},
//     { favorite: false, name: "input is limited to 32 characters!",                   func: -> (t, i, x, y) { (x-5)**2 + (y-5)**2 - 99*sin(t)                                }},
//   ]

//   def initialize
//     @preset_index = 0
//     @counter      = 0
//   end
//           if ev.scancode == SDL2::Key::Scan::Z
//             preset_change(1)
//           end
//           if ev.scancode == SDL2::Key::Scan::X
//             preset_change(-1)
//           end
//         end
//       end

fn my_sin(v: f32) -> f32 {
    v.sin()
}

fn my_rand() -> f32 {
    rand::thread_rng().gen_range(0.0, 1.0)
}

type MyFuncType = fn(f32, f32, f32, f32) -> f32;

// fn func_call(t: f32, i: f32, x: f32, y: f32) -> f32 {
//     let _ = t;
//     let _ = i;
//     let _ = x;
//     let _ = y;
//
//     // PRESET_LIST.0(t, i, x, y)
//
//     // // rand::thread_rng().gen_range(-1.0, 1.0)
//     // t.sin()
//
//     // if e = current_preset
//     //     v = instance_exec(t, Float(i), Float(x), Float(y), &e[:func])
//     //     if v == true
//     //     v = 1.0
//     //     elsif v == false || v.nil?
//     //     v = 0.0
//     //     end
//     //     v
//     //     end
//     //     end
// }

//
//   def current_preset
//     filtered_preset_list[@preset_index.modulo(filtered_preset_list.size)]
//   end
//
//   def filtered_preset_list
//     # @filtered_preset_list ||= PRESET_LIST.find_all { |e| e[:favorite] }
//     @filtered_preset_list ||= PRESET_LIST
//   end
//
//   def counter_reset
//     @counter = 0
//   end
//
// 0 は来ない
fn value_to_color(v: f32) -> (u8, u8, u8) {
    let v2: f32;
    if GRADATION_MODE {
        v2 = v
    } else {
        if v > 0.0 {
            v2 = 1.0
        } else {
            v2 = -1.0
        }
    }
    let c = (v2.abs() * COLOR_MAX as f32) as u8;
    if v2 > 0.0 {
        (c, c, c)
    } else {
        (0, c, 0)
    }
}
//
//   # 楕円の半径の割り合いを返す

fn value_to_radius_rate(rv: f32) -> f32 {
    rv.abs() * 0.9
}

#[derive(Default)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn scale(&self, value: f32) -> Vec2 {
        Vec2 { x: self.x * value, y: self.y * value }
    }

    fn add(&self, other: &Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }

    fn sub(&self, other: &Vec2) -> Vec2 {
        Vec2 { x: self.x - other.x, y: self.y - other.y }
    }
}

#[derive(Default)]
struct Application {
    srect: Vec2,
    cell_wh: Vec2,
    half_cell_wh: Vec2,
    top_left: Vec2,
    counter: usize,
    preset_index: isize,
}

impl Application {
    fn setup_vars(&mut self) {
        self.cell_wh      = self.srect.scale((1.0 / SIDE_SIZE) * VIEW_SIZE_RATE);            // 画面の大きさから1つのセルのサイズを求める
        self.half_cell_wh = self.cell_wh.scale(0.5);                                         // 扱いやすいように半分バージョンも作っておく
        self.top_left     = self.srect.scale(0.5).sub(&self.cell_wh.scale(SIDE_SIZE * 0.5)); // 左上
    }

    fn time(&self) -> f32 {
        self.counter as f32 / FPS as f32
    }

    fn preset_change(&mut self, sign: isize) {
        self.preset_index += sign
    }
}

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use rand::Rng;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut app = Application {
        srect: Vec2 { x: 800.0, y: 600.0 },
        ..Default::default()
    };
    app.setup_vars();

    let preset_list = [
        |t:f32, i:f32, x:f32, y:f32| { my_sin(t) },
        |t:f32, i:f32, x:f32, y:f32| { my_rand() },
    ];

    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;
    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                } |
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    app.preset_change(1)
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    app.preset_change(-1)
                },
                _ => {},
            }
        }
        // The rest of the game loop goes here...

        // canvas.set_draw_color(Color::RGB(255, 0, 0));
        // let rect = sdl2::rect::Rect::new(0, 0, 100, 100);
        // let _ = canvas.fill_rect(rect);

        let mut index = 0;
        for y in 0..SIDE_SIZE as usize {
            for x in 0..SIDE_SIZE as usize {
                let x = x as f32;
                let y = y as f32;

                // let mut retval = rand::thread_rng().gen_range(-1.0, 1.0);

                // println!("{:?}", app.time());

                // let mut retval = func_call(app.time(), index as f32, x as f32, y as f32);
                let idx = app.preset_index as usize % preset_list.len();
                let func = preset_list[idx as usize];
                let mut retval = func(app.time(), index as f32, x, y);

                if retval != 0.0 {
                    if retval < -1.0 {
                        retval = -1.0
                    }
                    if retval > 1.0 {
                        retval = 1.0
                    }
                    // let v = @top_left + @cell_wh.map2([x, y]) { |a, b| a * b }
                    let v = Vec2 { x: app.cell_wh.x * x, y: app.cell_wh.y * y };
                    let v = app.top_left.add(&v);

                    let radius = app.half_cell_wh.scale(value_to_radius_rate(retval));  // 楕円の半径 = 最大半径 * 割合
                    let center = v.add(&app.half_cell_wh);                               // セルの中心
                    let v2 = center.sub(&radius);                                           // 長方形の左上

                    let (r, g, b) = value_to_color(retval);
                    canvas.set_draw_color(Color::RGB(r, g, b));
                    let rect = sdl2::rect::Rect::new(v2.x as i32, v2.y as i32, (radius.x*2.0) as u32, (radius.y*2.0) as u32);
                    let _ = canvas.fill_rect(rect);

                    // @renderer.draw_color =
                    //         @renderer.fill_rect(SDL2::Rect.new(*v2, *(radius*2)))  # v2 から [radius, radius] の長方形を描画
                }
                index += 1;
            }
        }

        app.counter += 1;
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}
