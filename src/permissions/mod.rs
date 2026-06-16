pub enum Permission {
    Banned,
    Guest,
    User,
    Admin,
    SuperAdmin,
}

impl Permission {
    pub fn to_int(&self) -> i32 {
        match self {
            Permission::Banned => 0,
            Permission::Guest => 1,
            Permission::User => 2,
            Permission::Admin => 3,
            Permission::SuperAdmin => 4,
        }
    }

    pub fn from_int(num: i32) -> Self {
        match num {
            0 => Self::Banned,
            1 => Self::Guest,
            2 => Self::User,
            3 => Self::Admin,
            4 => Self::SuperAdmin,
            _ => Self::User,
        }
    }
}

impl TryFrom<i32> for Permission {
    type Error = &'static str;
    fn try_from(num: i32) -> Result<Self, Self::Error> {
        match num {
            0 => Ok(Self::Banned),
            1 => Ok(Self::Guest),
            2 => Ok(Self::User),
            3 => Ok(Self::Admin),
            4 => Ok(Self::SuperAdmin),
            _ => Err("invalid permission code"),
        }
    }
}
