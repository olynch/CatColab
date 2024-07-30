//! Standard library of double theories.

use std::hash::BuildHasherDefault;
use ustr::{ustr, IdentityHasher, Ustr};

use crate::dbl::theory::{DblTheory, DiscreteDblTheory, DiscreteTabTheory, TabObType};
use crate::one::fin_category::*;

type UstrDiscreteDblThy = DiscreteDblTheory<UstrFinCategory>;
type UstrDiscreteTabThy = DiscreteTabTheory<Ustr, Ustr, BuildHasherDefault<IdentityHasher>>;

/** The theory of categories, aka the trivial double theory.

As a double category, this is the terminal double category.
 */
pub fn th_category() -> UstrDiscreteDblThy {
    let mut cat: UstrFinCategory = Default::default();
    cat.add_ob_generator(ustr("object"));
    DiscreteDblTheory::from(cat)
}

/** The theory of database schemas with attributes.

As a double category, this is the "walking proarrow".
 */
pub fn th_schema() -> UstrDiscreteDblThy {
    let mut cat: UstrFinCategory = Default::default();
    let (x, y, p) = (ustr("entity"), ustr("attr_type"), ustr("attr"));
    cat.add_ob_generator(x);
    cat.add_ob_generator(y);
    cat.add_hom_generator(p, x, y);
    DiscreteDblTheory::from(cat)
}

/** The theory of signed categories.

A signed category is a category sliced over the group of signs.
 */
pub fn th_signed_category() -> UstrDiscreteDblThy {
    let mut sgn: UstrFinCategory = Default::default();
    let (x, n) = (ustr("object"), ustr("negative"));
    sgn.add_ob_generator(x);
    sgn.add_hom_generator(n, x, x);
    sgn.set_composite(n, n, FinHom::Id(x));
    DiscreteDblTheory::from(sgn)
}

/** The theory of categories with links.

A category with links is a category `C` together with a profunctor from `C` to
`Arr(C)`, the arrow category of C.
 */
pub fn th_category_links() -> UstrDiscreteTabThy {
    let mut th: UstrDiscreteTabThy = Default::default();
    let x = ustr("object");
    th.add_ob_type(x);
    th.add_mor_type(
        ustr("link"),
        TabObType::Basic(x),
        th.tabulator(th.hom_type(TabObType::Basic(x))),
    );
    th
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dbl::theory::DblTheory;

    #[test]
    fn theories() {
        assert_eq!(th_category().basic_ob_types().count(), 1);
        assert_eq!(th_schema().basic_ob_types().count(), 2);
        assert_eq!(th_signed_category().basic_mor_types().count(), 1);
        assert_eq!(th_category_links().basic_mor_types().count(), 1);
    }
}
