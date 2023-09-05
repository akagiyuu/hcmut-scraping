use similar::{capture_diff_slices, Algorithm, Change, ChangeTag, DiffOp};

pub fn diff(new_announcements: &[String], old_announcements: &[String]) -> Vec<Change<String>> {
    let ops = capture_diff_slices(Algorithm::Myers, old_announcements, new_announcements);
    let op = ops[0];
    if !matches!(
        op,
        DiffOp::Equal {
            old_index: _,
            new_index: _,
            len: _
        },
    ) {
        op
            .iter_changes(old_announcements, new_announcements)
            .filter(|change| (*change).tag() == ChangeTag::Insert)
            .collect()
    } else {
        vec![]
    }
}
