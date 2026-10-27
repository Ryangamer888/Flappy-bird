use std::{
  thread,
  time,
  io::{self,Write,StdoutLock},
};
use crossterm::{
  event::{self, Event, KeyCode},
  terminal::{enable_raw_mode, disable_raw_mode},
};

const SIZE: [i16; 2] = [50, 25];
struct Game{
  score:i32,
  state:i32,
  running:bool,
  pc:i32,
  i:String
}
struct Bird{
  y:f32,
  ym:f32,
  j:f32,
  g:f32
}
struct Pipe{
  x:i32,
  y:i32
}

fn draw(out: &mut StdoutLock,s: &str,x: i16,y: i16,c: i16) -> io::Result<()>{
  let f = format!("\x1b[{};{}H\x1b[{}m{}",y,x,c,s);
  out.write_all(f.as_bytes())?;
  Ok(())
}

fn render(t: &str,out: &mut StdoutLock,g: &mut Game, b: &mut Bird, p: &mut Vec<Pipe>) -> io::Result<()>{
  let mut y = SIZE[0];
  while y != 0 {
    draw(out,"#",y,0,37)?;
    draw(out,"#",y,SIZE[1],37)?;
    y -= 1;
  }
  y = SIZE[1];
  while y != 0 {
    draw(out,"#",0,y,37)?;
    draw(out,"#",SIZE[0],y,37)?;
    y -= 1;
  }
  if t == "Title" {
    draw(out,"Flappy Bird",20,5,31)?;
    if g.i == " " {
      *b = Bird{
        y: (SIZE[1] as f32) / 2.0,
        ym: 0.0,
        j: 1.5,
        g: 0.5
      };
      g.state = 1;
      g.score = 0;
      for i in 0..g.pc {
        p[i as usize].x = 50 + 25 * i; 
      }
    }
  }else{
    b.y -= b.ym;
    b.ym -= b.g;
    if g.i == " " {b.ym = b.j}
    draw(out,"@",5,b.y as i16,33)?;

    for i in p {
      i.x -= 1;
      if i.x<1 {
        i.x = SIZE[0] as i32;
        i.y = rand::random_range(5..=SIZE[1]-5) as i32;
        g.score += 1;
      }
      if i.x<SIZE[0] as i32 {
        for e in 2..=i.y-5 {
          draw(out,"#",i.x as i16,e as i16,32)?;
          if i.x == 5 && e == b.y as i32 {
            g.state = 0;
          }
        }
        for e in i.y+5..(SIZE[1] as i32) {
          draw(out,"#",i.x as i16,e as i16,32)?;
          if i.x == 5 && e == b.y as i32 {
            g.state = 0;
          }
        }
      }
    }

    if b.y as i16 > SIZE[1] || (b.y as i32) < 1 {
      g.state = 0;
    }

    draw(out,&g.score.to_string(),2,2,37)?;
  }
  Ok(())
}

fn main() -> io::Result<()>{
  enable_raw_mode()?;
  let mut g = Game{
    score: 0,
    state: 0,
    running: true,
    pc: 2,
    i: "".to_string()
  };
  let mut b = Bird{
    y:0.0,
    ym:0.0,
    j:0.0,
    g:0.0
  };
  let mut p: Vec<Pipe> = Vec::new();
  for i in 0..g.pc {
    p.push(Pipe{
      x: (SIZE[0] as i32) + (SIZE[0] as i32) / 2 * i,
      y: (rand::random_range(10..SIZE[1]-10) as i32)
    });
  }
  let stdout = io::stdout();
  let mut out = stdout.lock();
  
  while g.running {
    out.write_all(b"\x1b[2J")?;
    g.i = "".to_string();

    if event::poll(time::Duration::from_millis(100))? {
      if let Event::Key(key_event) = event::read()? {
        match key_event.code {
          KeyCode::Char(' ') => g.i = " ".to_string(),
          KeyCode::Esc => g.running = false,
          _ => {}
        }
      }
    }
    
    if g.state == 0 {
      render("Title",&mut out,&mut g, &mut b, &mut p)?;
    }else if g.state == 1{
      render("Game", &mut out, &mut g, &mut b, &mut p)?;
    }else{
      g.running = false;
    }
    out.flush()?;
    thread::sleep(time::Duration::from_millis(1000/60));
  }
  disable_raw_mode()?;
  Ok(())
}
