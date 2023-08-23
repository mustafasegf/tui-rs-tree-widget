use crate::identifier::{TreeIdentifier, TreeIdentifierVec};
use crate::TreeItem;

pub struct Flattened<'a, I> {
    pub identifier: Vec<usize>,
    pub item: &'a TreeItem<'a, I>,
}

impl<'a, I> Flattened<'a, I> {
    #[must_use]
    pub fn depth(&self) -> usize {
        self.identifier.len() - 1
    }
}

/// Get a flat list of all visible [`TreeItem`s](TreeItem)
#[must_use]
pub fn flatten<'a, I>(opened: &[TreeIdentifierVec], items: &'a [TreeItem<'a, I>]) -> Vec<Flattened<'a, I>> {
    internal(opened, items, &[])
}

#[must_use]
fn internal<'a, I>(
    opened: &[TreeIdentifierVec],
    items: &'a [TreeItem<'a, I>],
    current: TreeIdentifier,
) -> Vec<Flattened<'a, I>> {
    let mut result = Vec::new();

    for (index, item) in items.iter().enumerate() {
        let mut child_identifier = current.to_vec();
        child_identifier.push(index);

        result.push(Flattened {
            item,
            identifier: child_identifier.clone(),
        });

        if opened.contains(&child_identifier) {
            let mut child_result = internal(opened, &item.children, &child_identifier);
            result.append(&mut child_result);
        }
    }

    result
}

#[cfg(all(test, feature = "ratatui"))]
fn get_naive_string_from_text(text: &ratatui::text::Text<'_>) -> String {
    text.lines
        .first()
        .unwrap()
        .spans
        .first()
        .unwrap()
        .content
        .to_string()
}

#[cfg(all(test, not(feature = "ratatui")))]
fn get_naive_string_from_text(text: &tui::text::Text<'_>) -> String {
    text.lines
        .first()
        .unwrap()
        .0
        .first()
        .unwrap()
        .content
        .to_string()
}

#[cfg(test)]
fn get_example_tree_items() -> Vec<TreeItem<'static>> {
    vec![
        TreeItem::new_leaf_with_name("a"),
        TreeItem::new(
            "b",
            vec![
                TreeItem::new_leaf_with_name("c"),
                TreeItem::new("d", vec![TreeItem::new_leaf_with_name("e"), TreeItem::new_leaf_with_name("f")]),
                TreeItem::new_leaf_with_name("g"),
            ],
        ),
        TreeItem::new_leaf_with_name("h"),
    ]
}

#[test]
fn get_opened_nothing_opened_is_top_level() {
    let items = get_example_tree_items();
    let result = flatten(&[], &items);
    let result_text = result
        .iter()
        .map(|o| get_naive_string_from_text(&o.item.text))
        .collect::<Vec<_>>();
    assert_eq!(result_text, ["a", "b", "h"]);
}

#[test]
fn get_opened_wrong_opened_is_only_top_level() {
    let items = get_example_tree_items();
    let opened = [vec![0], vec![1, 1]];
    let result = flatten(&opened, &items);
    let result_text = result
        .iter()
        .map(|o| get_naive_string_from_text(&o.item.text))
        .collect::<Vec<_>>();
    assert_eq!(result_text, ["a", "b", "h"]);
}

#[test]
fn get_opened_one_is_opened() {
    let items = get_example_tree_items();
    let opened = [vec![1]];
    let result = flatten(&opened, &items);
    let result_text = result
        .iter()
        .map(|o| get_naive_string_from_text(&o.item.text))
        .collect::<Vec<_>>();
    assert_eq!(result_text, ["a", "b", "c", "d", "g", "h"]);
}

#[test]
fn get_opened_all_opened() {
    let items = get_example_tree_items();
    let opened = [vec![1], vec![1, 1]];
    let result = flatten(&opened, &items);
    let result_text = result
        .iter()
        .map(|o| get_naive_string_from_text(&o.item.text))
        .collect::<Vec<_>>();
    assert_eq!(result_text, ["a", "b", "c", "d", "e", "f", "g", "h"]);
}
