use core::fmt;

#[allow(dead_code)]
pub enum CardKind  {
    FIOK=7,
    FOOK=6,
    FH=5,
    TOK=4,
    TP=3,
    OP=2,
    HC=1,
}

impl fmt::Debug for CardKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           CardKind::FIOK => write!(f, "five of a kind"),
           CardKind::FOOK => write!(f, "four of a kind"),
           CardKind::FH => write!(f, "full house"),
           CardKind::TOK => write!(f, "three of a kind"),
           CardKind::TP => write!(f, "two pairs"),
           CardKind::OP => write!(f, "one pair"),
           CardKind::HC => write!(f, "high card"),
       }
    }
}


#[derive(Debug)]
pub struct Bid {
    pub card: String,
    pub bid: u32,
    pub card_kind: CardKind,
}

