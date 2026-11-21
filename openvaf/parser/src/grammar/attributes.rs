use super::*;
use crate::grammar::expressions::expr;

pub(super) fn attrs(p: &mut Parser, recovery: TokenSet) {
    while p.at(T!["(*"]) {
        attr_list(p, recovery)
    }
}

fn attr_list(p: &mut Parser, recovery: TokenSet) {
    let m = p.start();
    p.bump(T!["(*"]);

    while !p.at_ts(TokenSet::new(&[EOF, T!["*)"]]))
        && ((p.at_ts(ATTR_TY_KW) && p.nth(1) == IDENT) || !p.at_ts(recovery))
    {
        attr(p, recovery);

        if !p.at(T!["*)"]) {
            if !p.eat(T![;]) {
                p.expect_with(T![,], &[T![,], T![;], T!["*)"]]);
            }
        }
    }

    p.eat(T!["*)"]);
    m.complete(p, ATTR_LIST);
}

const ATTR_RECOVERY_SET: TokenSet = TokenSet::new(&[T!["(*"], T!["*)"], T![=], T![,], T![;]]);
const ATTR_TY_KW: TokenSet = TokenSet::new(&[INTEGER_KW, REAL_KW, STRING_KW]);

fn attr(p: &mut Parser, recovery: TokenSet) {
    let m = p.start();

    // Support Verilog-AMS / Spectre typed attribute declarations such as
    // `(* integer inherited_mfactor; *)`.
    if p.at_ts(ATTR_TY_KW) && p.nth(1) == IDENT {
        p.bump_any();
    }

    name_r(p, ATTR_RECOVERY_SET.union(recovery));
    if p.eat(T![=]) {
        expr(p);
    }
    m.complete(p, ATTR);
}
