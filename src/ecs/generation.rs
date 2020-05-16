#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Generation(i32);

pub(crate) trait ZeroableGeneration {
    fn id(&self) -> i32;
    fn is_alive(&self) -> bool;
    fn die(self) -> Generation;
    fn raise(self) -> Generation;
}

impl Generation {
    pub fn one() -> Self {
        Self(1)
    }

    pub fn new(id: i32) -> Result<Self, &'static str> {
        if id == 0 {
            return Err("Generation ID must be non-zero");
        }

        Ok(Self(id))
    }

    pub fn id(&self) -> i32 {
        self.0
    }

    pub fn is_alive(&self) -> bool {
        self.id() > 0
    }

    pub fn die(self) -> Self {
        assert!(self.is_alive());

        Self(-self.id())
    }

    pub fn raise(self) -> Self {
        assert!(!self.is_alive());

        Self(1 - self.id())
    }
}

impl ZeroableGeneration for Option<Generation> {
    fn id(&self) -> i32 {
        self.map(|gen| gen.id()).unwrap_or(0)
    }

    fn is_alive(&self) -> bool {
        self.id() > 0
    }

    fn die(self) -> Generation {
        assert!(self.is_alive());

        Generation(-self.id())
    }

    fn raise(self) -> Generation {
        assert!(!self.is_alive());

        Generation(1 - self.id())
    }
}
