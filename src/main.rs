use crate::Mod::{Alt, Control, Shift};
use core::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

const ESC: Key = Key("ESC");

const UP: Key = Key("UP");
const LEFT: Key = Key("LEFT");
const DOWN: Key = Key("DOWN");
const RIGHT: Key = Key("RGHT");
const SPACE: Key = Key("SPCE");
const INSERT: Key = Key("INS");
const DELETE: Key = Key("DELE");
const HOME: Key = Key("HOME");
const END: Key = Key("END");
const PAGE_UP: Key = Key("PGUP");
const PAGE_DOWN: Key = Key("PGDN");

const TILDE: Key = Key("TLDE");
const _1: Key = Key("AE01");
const _2: Key = Key("AE02");
const _3: Key = Key("AE03");
const _4: Key = Key("AE04");
const _5: Key = Key("AE05");
const _6: Key = Key("AE06");
const _7: Key = Key("AE07");
const _8: Key = Key("AE08");
const _9: Key = Key("AE09");
const _0: Key = Key("AE10");
const MINUS: Key = Key("AE11");
const EQUAL: Key = Key("AE12");
const BACKSPACE: Key = Key("BKSP");

const TAB: Key = Key("TAB");
const Q: Key = Key("AD01");
const W: Key = Key("AD02");
const E: Key = Key("AD03");
const R: Key = Key("AD04");
const T: Key = Key("AD05");
const Y: Key = Key("AD06");
const U: Key = Key("AD07");
const I: Key = Key("AD08");
const O: Key = Key("AD09");
const P: Key = Key("AD10");
const BRACKET_LEFT: Key = Key("AD11");
const BRACKET_RIGHT: Key = Key("AD12");
const BACKSLASH: Key = Key("BKSL");

const A: Key = Key("AC01");
const S: Key = Key("AC02");
const D: Key = Key("AC03");
const F: Key = Key("AC04");
const G: Key = Key("AC05");
const H: Key = Key("AC06");
const J: Key = Key("AC07");
const K: Key = Key("AC08");
const L: Key = Key("AC09");
const SEMICOLON: Key = Key("AC10");
const APOSTROPHE: Key = Key("AC11");
const RETURN: Key = Key("RTRN");

const Z: Key = Key("AB01");
const X: Key = Key("AB02");
const C: Key = Key("AB03");
const V: Key = Key("AB04");
const B: Key = Key("AB05");
const N: Key = Key("AB06");
const M: Key = Key("AB07");
const COMMA: Key = Key("AB08");
const PERIOD: Key = Key("AB09");
const SLASH: Key = Key("AB10");

#[derive(Copy, Clone)]
struct Key(&'static str);

impl Display for Key {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "<{}>", self.0)
  }
}

#[derive(Copy, Clone)]
enum Mod {
  Control,
  Shift,
  Alt,
}

impl Display for Mod {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Control => write!(f, "Control"),
      Shift => write!(f, "Shift"),
      Alt => write!(f, "Alt"),
    }
  }
}

#[derive(Clone)]
struct Redirect {
  key: Key,
  mods: Vec<Mod>,
}

impl Redirect {
  fn new(key: Key, mods: Vec<Mod>) -> Redirect {
    Redirect { key, mods }
  }
}

impl Display for Redirect {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    if self.mods.is_empty() {
      write!(f, "Redirect(key={}, clearmods=All)", self.key)
    } else {
      write!(
        f,
        "Redirect(key={}, clearmods=All, mods={})",
        self.key,
        self
          .mods
          .iter()
          .map(ToString::to_string)
          .collect::<Vec<String>>()
          .join("+")
      )
    }
  }
}

struct EightLevelKey {
  key: Key,
  lv1: &'static str,
  lv2: &'static str,
  lv3: Option<Redirect>,
  lv4: Option<Redirect>,
  lv5: Option<Redirect>,
  lv6: Option<Redirect>,
  lv7: Option<Redirect>,
  lv8: Option<Redirect>,
}

impl EightLevelKey {
  fn lv2(key: Key, lv1: &'static str, lv2: &'static str) -> EightLevelKey {
    EightLevelKey {
      key,
      lv1,
      lv2,
      lv3: None,
      lv4: None,
      lv5: None,
      lv6: None,
      lv7: None,
      lv8: None,
    }
  }

  fn lv(&self, lv: &Option<Redirect>, mods: impl FnOnce() -> Vec<Mod>) -> Redirect {
    lv.clone().unwrap_or_else(|| Redirect {
      key: self.key,
      mods: mods(),
    })
  }

  fn lv3(&self) -> Redirect {
    self.lv(&self.lv3, || vec![Control])
  }

  fn lv4(&self) -> Redirect {
    self.lv(&self.lv4, || vec![Control, Shift])
  }

  fn lv5(&self) -> Redirect {
    self.lv(&self.lv5, || vec![Alt])
  }

  fn lv6(&self) -> Redirect {
    self.lv(&self.lv6, || vec![Alt, Shift])
  }

  fn lv7(&self) -> Redirect {
    self.lv(&self.lv7, || vec![Control, Alt])
  }

  fn lv8(&self) -> Redirect {
    self.lv(&self.lv8, || vec![Control, Alt, Shift])
  }
}

impl Display for EightLevelKey {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      r#"key {key} {{
  type="EIGHT_LEVEL",
  symbols=[{lv1}, {lv2}],
  actions=[
    NoAction(),
    NoAction(),
    {lv3},
    {lv4},
    {lv5},
    {lv6},
    {lv7},
    {lv8}
  ]
}};"#,
      key = self.key,
      lv1 = self.lv1,
      lv2 = self.lv2,
      lv3 = self.lv3(),
      lv4 = self.lv4(),
      lv5 = self.lv5(),
      lv6 = self.lv6(),
      lv7 = self.lv7(),
      lv8 = self.lv8(),
    )
  }
}

fn get_keys() -> Vec<EightLevelKey> {
  vec![
    EightLevelKey::lv2(BACKSLASH, "backslash", "bar"),
    EightLevelKey::lv2(SPACE, "space", "space"),
    EightLevelKey {
      lv3: Some(Redirect::new(BACKSPACE, vec![Control, Shift])),
      lv5: Some(Redirect::new(BACKSPACE, vec![Control])),
      ..EightLevelKey::lv2(BACKSPACE, "BackSpace", "BackSpace")
    },
    EightLevelKey::lv2(TAB, "Tab", "ISO_Left_Tab"),
    EightLevelKey::lv2(RETURN, "Return", "Return"),
    EightLevelKey::lv2(INSERT, "Insert", "Insert"),
    EightLevelKey {
      lv3: Some(Redirect::new(DELETE, vec![Control, Shift])),
      lv5: Some(Redirect::new(DELETE, vec![Control])),
      ..EightLevelKey::lv2(DELETE, "Delete", "Delete")
    },
    EightLevelKey::lv2(HOME, "Home", "Home"),
    EightLevelKey::lv2(END, "End", "End"),
    EightLevelKey::lv2(PAGE_UP, "Prior", "Prior"),
    EightLevelKey::lv2(PAGE_DOWN, "Next", "Next"),
    EightLevelKey {
      lv3: Some(Redirect::new(HOME, vec![Control])),
      lv4: Some(Redirect::new(HOME, vec![Control, Shift])),
      lv5: Some(Redirect::new(PAGE_UP, vec![])),
      lv6: Some(Redirect::new(PAGE_UP, vec![Shift])),
      ..EightLevelKey::lv2(UP, "Up", "Up")
    },
    EightLevelKey {
      lv3: Some(Redirect::new(HOME, vec![])),
      lv4: Some(Redirect::new(HOME, vec![Shift])),
      lv5: Some(Redirect::new(LEFT, vec![Control])),
      lv6: Some(Redirect::new(LEFT, vec![Control, Shift])),
      ..EightLevelKey::lv2(LEFT, "Left", "Left")
    },
    EightLevelKey {
      lv3: Some(Redirect::new(END, vec![Control])),
      lv4: Some(Redirect::new(END, vec![Control, Shift])),
      lv5: Some(Redirect::new(PAGE_DOWN, vec![])),
      lv6: Some(Redirect::new(PAGE_DOWN, vec![Shift])),
      ..EightLevelKey::lv2(DOWN, "Down", "Down")
    },
    EightLevelKey {
      lv3: Some(Redirect::new(END, vec![])),
      lv4: Some(Redirect::new(END, vec![Shift])),
      lv5: Some(Redirect::new(RIGHT, vec![Control])),
      lv6: Some(Redirect::new(RIGHT, vec![Control, Shift])),
      ..EightLevelKey::lv2(RIGHT, "Right", "Right")
    },
    EightLevelKey::lv2(TILDE, "grave", "asciitilde"),
    EightLevelKey::lv2(_1, "1", "exclam"),
    EightLevelKey::lv2(_2, "2", "at"),
    EightLevelKey::lv2(_3, "3", "numbersign"),
    EightLevelKey::lv2(_4, "4", "dollar"),
    EightLevelKey::lv2(_5, "5", "percent"),
    EightLevelKey::lv2(_6, "6", "asciicircum"),
    EightLevelKey::lv2(_7, "7", "ampersand"),
    EightLevelKey::lv2(_8, "8", "asterisk"),
    EightLevelKey::lv2(_9, "9", "parenleft"),
    EightLevelKey::lv2(_0, "0", "parenright"),
    EightLevelKey::lv2(MINUS, "minus", "underscore"),
    EightLevelKey::lv2(EQUAL, "equal", "plus"),
    EightLevelKey::lv2(Q, "q", "Q"),
    EightLevelKey::lv2(W, "w", "W"),
    EightLevelKey {
      lv3: Some(Redirect::new(END, vec![])),
      lv4: Some(Redirect::new(END, vec![Shift])),
      ..EightLevelKey::lv2(E, "e", "E")
    },
    EightLevelKey::lv2(R, "r", "R"),
    EightLevelKey::lv2(T, "t", "T"),
    EightLevelKey::lv2(Y, "y", "Y"),
    EightLevelKey::lv2(U, "u", "U"),
    EightLevelKey::lv2(I, "i", "I"),
    EightLevelKey::lv2(O, "o", "O"),
    EightLevelKey {
      lv3: Some(Redirect::new(UP, vec![])),
      lv4: Some(Redirect::new(UP, vec![Shift])),
      lv5: Some(Redirect::new(PAGE_UP, vec![])),
      lv6: Some(Redirect::new(PAGE_UP, vec![Shift])),
      ..EightLevelKey::lv2(P, "p", "P")
    },
    EightLevelKey::lv2(BRACKET_LEFT, "bracketleft", "braceleft"),
    EightLevelKey::lv2(BRACKET_RIGHT, "bracketright", "braceright"),
    EightLevelKey {
      lv3: Some(Redirect::new(HOME, vec![])),
      lv4: Some(Redirect::new(HOME, vec![Shift])),
      ..EightLevelKey::lv2(A, "a", "A")
    },
    EightLevelKey::lv2(S, "s", "S"),
    EightLevelKey {
      lv3: Some(Redirect::new(DELETE, vec![])),
      lv5: Some(Redirect::new(DELETE, vec![Control])),
      ..EightLevelKey::lv2(D, "d", "D")
    },
    EightLevelKey {
      lv3: Some(Redirect::new(RIGHT, vec![])),
      lv4: Some(Redirect::new(RIGHT, vec![Shift])),
      lv5: Some(Redirect::new(RIGHT, vec![Control])),
      lv6: Some(Redirect::new(RIGHT, vec![Control, Shift])),
      ..EightLevelKey::lv2(F, "f", "F")
    },
    EightLevelKey {
      lv3: Some(Redirect::new(ESC, vec![])),
      ..EightLevelKey::lv2(G, "g", "G")
    },
    EightLevelKey::lv2(H, "h", "H"),
    EightLevelKey::lv2(J, "j", "J"),
    EightLevelKey::lv2(K, "k", "K"),
    EightLevelKey::lv2(L, "l", "L"),
    EightLevelKey::lv2(SEMICOLON, "semicolon", "colon"),
    EightLevelKey::lv2(APOSTROPHE, "apostrophe", "quotedbl"),
    EightLevelKey::lv2(Z, "z", "Z"),
    EightLevelKey::lv2(X, "x", "X"),
    EightLevelKey::lv2(C, "c", "C"),
    EightLevelKey::lv2(V, "v", "V"),
    EightLevelKey {
      lv3: Some(Redirect::new(LEFT, vec![])),
      lv4: Some(Redirect::new(LEFT, vec![Shift])),
      lv5: Some(Redirect::new(LEFT, vec![Control])),
      lv6: Some(Redirect::new(LEFT, vec![Control, Shift])),
      ..EightLevelKey::lv2(B, "b", "B")
    },
    EightLevelKey {
      lv3: Some(Redirect::new(DOWN, vec![])),
      lv4: Some(Redirect::new(DOWN, vec![Shift])),
      lv5: Some(Redirect::new(PAGE_DOWN, vec![])),
      lv6: Some(Redirect::new(PAGE_DOWN, vec![Shift])),
      ..EightLevelKey::lv2(N, "n", "N")
    },
    EightLevelKey::lv2(M, "m", "M"),
    EightLevelKey {
      lv6: Some(Redirect::new(HOME, vec![Control])),
      ..EightLevelKey::lv2(COMMA, "comma", "less")
    },
    EightLevelKey {
      lv6: Some(Redirect::new(END, vec![Control])),
      ..EightLevelKey::lv2(PERIOD, "period", "greater")
    },
    EightLevelKey::lv2(SLASH, "slash", "question"),
  ]
}

fn main() -> io::Result<()> {
  let mut writer = BufWriter::new(File::create("custom.xkb")?);

  writeln!(
    writer,
    r#"// Generated
partial alphanumeric_keys modifier_keys
xkb_symbols "basic" {{

  include "us"
  include "level5(modifier_mapping)""#
  )?;

  for key in get_keys().iter() {
    writeln!(writer, "")?;
    for line in format!("{}", key).lines() {
      writeln!(writer, "  {}", line)?;
    }
  }
  writeln!(writer, "}};")?;

  writeln!(
    writer,
    r#"
partial modifier_keys
xkb_symbols "lalt_as_lv3" {{
  key <LALT> {{[ISO_Level3_Shift, ISO_Level3_Shift]}};
}};

partial modifier_keys
xkb_symbols "lwin_as_lv5" {{
  key <LWIN> {{[ISO_Level5_Shift]}};
}};

partial modifier_keys
xkb_symbols "lctrl_as_lwin" {{
  key <LCTL> {{[Super_L]}};
  modifier_map Mod4 {{<LCTL>}};
}};

partial modifier_keys
xkb_symbols "ralt_as_rctrl" {{
  key <RALT> {{[Control_R, Control_R]}};
  modifier_map Control {{<RALT>}};
}};

partial modifier_keys
xkb_symbols "prtsc_as_ralt" {{
  key <PRSC> {{[Alt_R, Meta_R]}};
  modifier_map Mod1 {{<PRSC>}};
}};

partial modifier_keys
xkb_symbols "rwin_as_ralt" {{
  key <RWIN> {{[Alt_R, Meta_R]}};
  modifier_map Mod1 {{<RWIN>}};
}};

partial modifier_keys
xkb_symbols "rctrl_as_rwin" {{
  key <RCTL> {{[Super_R]}};
  modifier_map Mod4 {{<RCTL>}};
}};
"#
  )?;

  Ok(())
}
