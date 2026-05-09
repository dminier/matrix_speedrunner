use rand::Rng;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

// IMPORTANT : uniquement des caractères de largeur 1 cellule.
// Les katakana pleine-largeur (アイウ…) et le grec sont AMBIGUOUS/WIDE selon
// le terminal et provoquent des cellules fantômes (ratatui ne marque pas la
// continuation), d'où des artefacts de rendu persistants frame après frame.
const GLYPHS: &str = concat!(
    "0123456789",
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    "abcdefghijklmnopqrstuvwxyz",
    "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜｦﾝ",
    "ｧｨｩｪｫｬｭｮｯｰﾞﾟ･",
    "+-*/=<>!?@#$%&|^~:;.,",
);

pub struct Column {
    pub head_y: f32,
    pub speed: f32,
    pub length: u16,
    pub chars: Vec<char>,
}

pub struct Rain {
    pub w: u16,
    pub h: u16,
    pub columns: Vec<Column>,
}

impl Rain {
    pub fn new(w: u16, h: u16) -> Self {
        let mut r = Self {
            w,
            h,
            columns: Vec::new(),
        };
        r.resize(w, h);
        r
    }

    pub fn resize(&mut self, w: u16, h: u16) {
        self.w = w.max(1);
        self.h = h.max(1);
        let mut rng = rand::thread_rng();
        self.columns = (0..self.w)
            .map(|_| Self::fresh_column(&mut rng, self.h))
            .collect();
    }

    fn fresh_column<R: Rng>(rng: &mut R, h: u16) -> Column {
        let length = rng.gen_range(4..h.max(5).min(20));
        let chars: Vec<char> = (0..h).map(|_| random_glyph(rng)).collect();
        Column {
            head_y: rng.gen_range(-(h as f32)..0.0),
            speed: rng.gen_range(5.0..20.0),
            length,
            chars,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        let mut rng = rand::thread_rng();
        let h = self.h;
        for col in &mut self.columns {
            col.head_y += col.speed * dt;
            if col.head_y > h as f32 + col.length as f32 {
                *col = Self::fresh_column(&mut rng, h);
                continue;
            }
            // shimmer aléatoire pour donner cet effet "scintillement".
            if rng.gen_bool(0.05) {
                let i = rng.gen_range(0..col.chars.len());
                col.chars[i] = random_glyph(&mut rng);
            }
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // Tête : vert clair mais bien VERT (pas blanc cassé).
        // Sur des terminaux sans truecolor, (180,255,200) tombe en "white"
        // ANSI 16-couleurs → ça donne un caractère blanc dans la pluie.
        let bright = Color::Rgb(140, 255, 160);
        let mid = Color::Rgb(0, 255, 65);
        let dim = Color::Rgb(0, 100, 30);
        for (cx, col) in self.columns.iter().enumerate() {
            if cx as u16 >= area.width {
                break;
            }
            let head = col.head_y;
            for i in 0..col.length {
                let y = head - i as f32;
                if y < 0.0 || y >= area.height as f32 {
                    continue;
                }
                let yi = y as u16;
                let ch = col.chars[(yi as usize) % col.chars.len()];
                let color = if i == 0 {
                    bright
                } else if i < col.length / 3 {
                    mid
                } else {
                    dim
                };
                let cell = &mut buf[(area.x + cx as u16, area.y + yi)];
                cell.set_char(ch);
                cell.set_style(Style::default().fg(color));
            }
        }
    }
}

fn random_glyph<R: Rng>(rng: &mut R) -> char {
    let chars: Vec<char> = GLYPHS.chars().collect();
    chars[rng.gen_range(0..chars.len())]
}
