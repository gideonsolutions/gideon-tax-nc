/// All 100 counties in North Carolina.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum County {
    Alamance,
    Alexander,
    Alleghany,
    Anson,
    Ashe,
    Avery,
    Beaufort,
    Bertie,
    Bladen,
    Brunswick,
    Buncombe,
    Burke,
    Cabarrus,
    Caldwell,
    Camden,
    Carteret,
    Caswell,
    Catawba,
    Chatham,
    Cherokee,
    Chowan,
    Clay,
    Cleveland,
    Columbus,
    Craven,
    Cumberland,
    Currituck,
    Dare,
    Davidson,
    Davie,
    Duplin,
    Durham,
    Edgecombe,
    Forsyth,
    Franklin,
    Gaston,
    Gates,
    Graham,
    Granville,
    Greene,
    Guilford,
    Halifax,
    Harnett,
    Haywood,
    Henderson,
    Hertford,
    Hoke,
    Hyde,
    Iredell,
    Jackson,
    Johnston,
    Jones,
    Lee,
    Lenoir,
    Lincoln,
    Macon,
    Madison,
    Martin,
    McDowell,
    Mecklenburg,
    Mitchell,
    Montgomery,
    Moore,
    Nash,
    NewHanover,
    Northampton,
    Onslow,
    Orange,
    Pamlico,
    Pasquotank,
    Pender,
    Perquimans,
    Person,
    Pitt,
    Polk,
    Randolph,
    Richmond,
    Robeson,
    Rockingham,
    Rowan,
    Rutherford,
    Sampson,
    Scotland,
    Stanly,
    Stokes,
    Surry,
    Swain,
    Transylvania,
    Tyrrell,
    Union,
    Vance,
    Wake,
    Warren,
    Washington,
    Watauga,
    Wayne,
    Wilkes,
    Wilson,
    Yadkin,
    Yancey,
}

impl County {
    /// Returns the full name of the county.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Alamance => "Alamance",
            Self::Alexander => "Alexander",
            Self::Alleghany => "Alleghany",
            Self::Anson => "Anson",
            Self::Ashe => "Ashe",
            Self::Avery => "Avery",
            Self::Beaufort => "Beaufort",
            Self::Bertie => "Bertie",
            Self::Bladen => "Bladen",
            Self::Brunswick => "Brunswick",
            Self::Buncombe => "Buncombe",
            Self::Burke => "Burke",
            Self::Cabarrus => "Cabarrus",
            Self::Caldwell => "Caldwell",
            Self::Camden => "Camden",
            Self::Carteret => "Carteret",
            Self::Caswell => "Caswell",
            Self::Catawba => "Catawba",
            Self::Chatham => "Chatham",
            Self::Cherokee => "Cherokee",
            Self::Chowan => "Chowan",
            Self::Clay => "Clay",
            Self::Cleveland => "Cleveland",
            Self::Columbus => "Columbus",
            Self::Craven => "Craven",
            Self::Cumberland => "Cumberland",
            Self::Currituck => "Currituck",
            Self::Dare => "Dare",
            Self::Davidson => "Davidson",
            Self::Davie => "Davie",
            Self::Duplin => "Duplin",
            Self::Durham => "Durham",
            Self::Edgecombe => "Edgecombe",
            Self::Forsyth => "Forsyth",
            Self::Franklin => "Franklin",
            Self::Gaston => "Gaston",
            Self::Gates => "Gates",
            Self::Graham => "Graham",
            Self::Granville => "Granville",
            Self::Greene => "Greene",
            Self::Guilford => "Guilford",
            Self::Halifax => "Halifax",
            Self::Harnett => "Harnett",
            Self::Haywood => "Haywood",
            Self::Henderson => "Henderson",
            Self::Hertford => "Hertford",
            Self::Hoke => "Hoke",
            Self::Hyde => "Hyde",
            Self::Iredell => "Iredell",
            Self::Jackson => "Jackson",
            Self::Johnston => "Johnston",
            Self::Jones => "Jones",
            Self::Lee => "Lee",
            Self::Lenoir => "Lenoir",
            Self::Lincoln => "Lincoln",
            Self::Macon => "Macon",
            Self::Madison => "Madison",
            Self::Martin => "Martin",
            Self::McDowell => "McDowell",
            Self::Mecklenburg => "Mecklenburg",
            Self::Mitchell => "Mitchell",
            Self::Montgomery => "Montgomery",
            Self::Moore => "Moore",
            Self::Nash => "Nash",
            Self::NewHanover => "New Hanover",
            Self::Northampton => "Northampton",
            Self::Onslow => "Onslow",
            Self::Orange => "Orange",
            Self::Pamlico => "Pamlico",
            Self::Pasquotank => "Pasquotank",
            Self::Pender => "Pender",
            Self::Perquimans => "Perquimans",
            Self::Person => "Person",
            Self::Pitt => "Pitt",
            Self::Polk => "Polk",
            Self::Randolph => "Randolph",
            Self::Richmond => "Richmond",
            Self::Robeson => "Robeson",
            Self::Rockingham => "Rockingham",
            Self::Rowan => "Rowan",
            Self::Rutherford => "Rutherford",
            Self::Sampson => "Sampson",
            Self::Scotland => "Scotland",
            Self::Stanly => "Stanly",
            Self::Stokes => "Stokes",
            Self::Surry => "Surry",
            Self::Swain => "Swain",
            Self::Transylvania => "Transylvania",
            Self::Tyrrell => "Tyrrell",
            Self::Union => "Union",
            Self::Vance => "Vance",
            Self::Wake => "Wake",
            Self::Warren => "Warren",
            Self::Washington => "Washington",
            Self::Watauga => "Watauga",
            Self::Wayne => "Wayne",
            Self::Wilkes => "Wilkes",
            Self::Wilson => "Wilson",
            Self::Yadkin => "Yadkin",
            Self::Yancey => "Yancey",
        }
    }

    /// Returns the first 5 characters of the county name (or fewer if shorter), uppercased.
    pub fn first_five(self) -> String {
        let name = self.name();
        let s: String = name.chars().take(5).collect();
        s.to_uppercase()
    }

    /// Returns a slice of all 100 NC counties.
    pub const fn all() -> &'static [County; 100] {
        &[
            Self::Alamance,
            Self::Alexander,
            Self::Alleghany,
            Self::Anson,
            Self::Ashe,
            Self::Avery,
            Self::Beaufort,
            Self::Bertie,
            Self::Bladen,
            Self::Brunswick,
            Self::Buncombe,
            Self::Burke,
            Self::Cabarrus,
            Self::Caldwell,
            Self::Camden,
            Self::Carteret,
            Self::Caswell,
            Self::Catawba,
            Self::Chatham,
            Self::Cherokee,
            Self::Chowan,
            Self::Clay,
            Self::Cleveland,
            Self::Columbus,
            Self::Craven,
            Self::Cumberland,
            Self::Currituck,
            Self::Dare,
            Self::Davidson,
            Self::Davie,
            Self::Duplin,
            Self::Durham,
            Self::Edgecombe,
            Self::Forsyth,
            Self::Franklin,
            Self::Gaston,
            Self::Gates,
            Self::Graham,
            Self::Granville,
            Self::Greene,
            Self::Guilford,
            Self::Halifax,
            Self::Harnett,
            Self::Haywood,
            Self::Henderson,
            Self::Hertford,
            Self::Hoke,
            Self::Hyde,
            Self::Iredell,
            Self::Jackson,
            Self::Johnston,
            Self::Jones,
            Self::Lee,
            Self::Lenoir,
            Self::Lincoln,
            Self::Macon,
            Self::Madison,
            Self::Martin,
            Self::McDowell,
            Self::Mecklenburg,
            Self::Mitchell,
            Self::Montgomery,
            Self::Moore,
            Self::Nash,
            Self::NewHanover,
            Self::Northampton,
            Self::Onslow,
            Self::Orange,
            Self::Pamlico,
            Self::Pasquotank,
            Self::Pender,
            Self::Perquimans,
            Self::Person,
            Self::Pitt,
            Self::Polk,
            Self::Randolph,
            Self::Richmond,
            Self::Robeson,
            Self::Rockingham,
            Self::Rowan,
            Self::Rutherford,
            Self::Sampson,
            Self::Scotland,
            Self::Stanly,
            Self::Stokes,
            Self::Surry,
            Self::Swain,
            Self::Transylvania,
            Self::Tyrrell,
            Self::Union,
            Self::Vance,
            Self::Wake,
            Self::Warren,
            Self::Washington,
            Self::Watauga,
            Self::Wayne,
            Self::Wilkes,
            Self::Wilson,
            Self::Yadkin,
            Self::Yancey,
        ]
    }
}

impl core::fmt::Display for County {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn there_are_100_counties() {
        assert_eq!(County::all().len(), 100);
    }

    #[test]
    fn first_five_short_names_are_full_name_uppercased() {
        assert_eq!(County::Ashe.first_five(), "ASHE");
        assert_eq!(County::Clay.first_five(), "CLAY");
        assert_eq!(County::Dare.first_five(), "DARE");
        assert_eq!(County::Hoke.first_five(), "HOKE");
        assert_eq!(County::Hyde.first_five(), "HYDE");
        assert_eq!(County::Lee.first_five(), "LEE");
        assert_eq!(County::Nash.first_five(), "NASH");
        assert_eq!(County::Pitt.first_five(), "PITT");
        assert_eq!(County::Polk.first_five(), "POLK");
        assert_eq!(County::Wake.first_five(), "WAKE");
    }

    #[test]
    fn first_five_long_names_are_truncated_uppercased() {
        assert_eq!(County::Mecklenburg.first_five(), "MECKL");
        assert_eq!(County::Cumberland.first_five(), "CUMBE");
        assert_eq!(County::Transylvania.first_five(), "TRANS");
        assert_eq!(County::Rockingham.first_five(), "ROCKI");
    }

    #[test]
    fn first_five_exact_five_char_names_uppercased() {
        assert_eq!(County::Burke.first_five(), "BURKE");
        assert_eq!(County::Avery.first_five(), "AVERY");
        assert_eq!(County::Anson.first_five(), "ANSON");
        assert_eq!(County::Moore.first_five(), "MOORE");
        assert_eq!(County::Union.first_five(), "UNION");
        assert_eq!(County::Vance.first_five(), "VANCE");
        assert_eq!(County::Wayne.first_five(), "WAYNE");
        assert_eq!(County::Surry.first_five(), "SURRY");
        assert_eq!(County::Swain.first_five(), "SWAIN");
        assert_eq!(County::Rowan.first_five(), "ROWAN");
    }

    #[test]
    fn new_hanover_first_five_includes_space_uppercased() {
        assert_eq!(County::NewHanover.first_five(), "NEW H");
    }

    #[test]
    fn display_matches_name() {
        assert_eq!(County::Wake.to_string(), "Wake");
        assert_eq!(County::NewHanover.to_string(), "New Hanover");
        assert_eq!(County::McDowell.to_string(), "McDowell");
    }
}
