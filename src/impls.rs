use crate::{Token as Sk, testing};

macro_rules! imp {
    ($f:path, $to:path) => {
        impl TryFrom<Sk> for $to {
            type Error = ();

            fn try_from(value: Sk) -> Result<Self, Self::Error> {
                if value == $f { Ok($to) } else { Err(()) }
            }
        }
    };
    ($to:path) => {
        impl TryFrom<Sk> for $to {
            type Error = ();

            fn try_from(_value: Sk) -> Result<Self, Self::Error> {
                Err(())
            }
        }
    };
}

imp!(Sk::COMMA, testing::Comma);
imp!(Sk::STOP, testing::Stop);
imp!(Sk::SEMI, testing::Colon);
imp!(Sk::L_SQ, testing::SqOpen);
imp!(Sk::R_SQ, testing::SqClose);

imp!(testing::TypeA);
imp!(testing::ClOpen);
imp!(testing::ClClose);
imp!(testing::Datatype);
imp!(testing::TrueValue);
imp!(testing::FalseValue);
imp!(testing::PrefixToken);
imp!(testing::BaseToken);
imp!(testing::SparqlBaseToken);
imp!(testing::SparqlPrefixToken);
