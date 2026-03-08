/// Calculate the complexity of a boolean expression.
///
/// Complexity is defined as the count of logical AND and OR operators
/// in the expression.
///
/// Note: ANDs within BETWEEN clauses (e.g., "BETWEEN 20 AND 30") are not
/// counted as they are part of the SQL syntax, not logical operators.
pub fn calculate_complexity(expr: &str) -> usize {
    let mut expr_upper = expr.to_uppercase();

    // Remove ANDs that are part of BETWEEN clauses
    // Pattern: "BETWEEN <value> AND <value>" or "NOT BETWEEN <value> AND <value>"
    // We'll replace both the BETWEEN keyword and the AND within it with placeholders

    // Find and process all BETWEEN clauses
    loop {
        if let Some(between_pos) = expr_upper.find(" BETWEEN ") {
            // Find the next " AND " after this BETWEEN
            let search_start = between_pos + " BETWEEN ".len();
            if let Some(and_pos) = expr_upper[search_start..].find(" AND ") {
                let actual_and_pos = search_start + and_pos;
                // Replace the AND within BETWEEN with a placeholder
                expr_upper.replace_range(actual_and_pos..actual_and_pos + 5, " ___ ");
                // Also replace BETWEEN keyword so we don't find it again in the next iteration
                expr_upper.replace_range(between_pos..between_pos + 10, " _______ ");
            } else {
                break;
            }
        } else {
            break;
        }
    }

    // Count " AND " and " OR " as whole words to avoid false positives
    // in strings like "BANANA" or "ORANGE"
    let and_count = expr_upper.matches(" AND ").count();
    let or_count = expr_upper.matches(" OR ").count();

    and_count + or_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_expressions() {
        assert_eq!(calculate_complexity("a = 1 AND b = 2"), 1);
        assert_eq!(calculate_complexity("a = 1 OR b = 2"), 1);
        assert_eq!(calculate_complexity("a = 1"), 0);
    }

    #[test]
    fn test_complex_expressions() {
        assert_eq!(calculate_complexity("a = 1 AND b = 2 OR c = 3"), 2);
        assert_eq!(calculate_complexity("a = 1 AND b = 2 AND c = 3 OR d = 4"), 3);
    }

    #[test]
    fn test_no_false_positives() {
        // "BANANA" contains "AND" but shouldn't match
        assert_eq!(calculate_complexity("s = 'BANANA' OR s = 'ORANGE'"), 1);
        assert_eq!(calculate_complexity("s = 'BANANA'"), 0);
    }

    #[test]
    fn test_between_clause() {
        // AND within BETWEEN should not be counted
        assert_eq!(calculate_complexity("f92 BETWEEN 20 AND 30"), 0);
        assert_eq!(calculate_complexity("f92 NOT BETWEEN 20 AND 30"), 0);
    }

    #[test]
    fn test_between_with_logical_operators() {
        // BETWEEN clause AND + logical AND
        assert_eq!(
            calculate_complexity("(f92 BETWEEN 20 AND 30) AND (x = 5)"),
            1
        );

        // BETWEEN clause AND + logical OR
        assert_eq!(
            calculate_complexity("(f92 NOT BETWEEN 20 AND 30) OR (x = 5)"),
            1
        );

        // Multiple BETWEEN clauses with logical operators
        assert_eq!(
            calculate_complexity("(f92 BETWEEN 20 AND 30) AND (f93 BETWEEN 40 AND 50)"),
            1
        );

        // Complex: BETWEEN + multiple logical operators
        assert_eq!(
            calculate_complexity("(f92 BETWEEN 20 AND 30) AND (x = 5) OR (y = 10)"),
            2
        );
    }

    #[test]
    fn test_real_world_examples() {
        // From the user's example
        assert_eq!(
            calculate_complexity("(s76 NOT LIKE '_sometext') AND (f92 NOT BETWEEN 20 AND 30)"),
            1 // Only the logical AND, not the BETWEEN AND
        );

        // Complex expression with BETWEEN
        assert_eq!(
            calculate_complexity(
                "(s69 <= 'banana') OR (s57 < 'banana') AND (i BETWEEN 1 AND 10)"
            ),
            2 // One OR and one logical AND, BETWEEN AND not counted
        );
    }
}
