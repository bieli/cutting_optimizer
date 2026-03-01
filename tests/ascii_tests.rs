use cutting_optimizer::engine::ascii_render;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_headers_present() {
        let out = ascii_render(&[50, 30], &[100, 100], &[vec![50], vec![30]]);

        assert!(out.contains("=== ASCII VISUALIZATION ==="));
        assert!(out.contains("Cuts in order:"));
        assert!(out.contains("=== RODS USED ==="));
    }

    #[test]
    fn test_ascii_full_usage_has_only_hashes() {
        let out = ascii_render(&[100], &[100], &[vec![100]]);

        assert!(out.contains("|##################################################|"));
    }

    #[test]
    fn test_ascii_leftover_has_dots() {
        let out = ascii_render(&[60], &[100], &[vec![60]]);

        assert!(out.contains("|##############################....................|"));
    }

    #[test]
    fn test_ascii_multiple_rods() {
        let out = ascii_render(
            &[50, 40, 10],
            &[100, 80, 20],
            &[vec![50], vec![40], vec![10]],
        );

        assert!(out.contains("Rod 1: 100 mm"));
        assert!(out.contains("Rod 2: 80 mm"));
        assert!(out.contains("Rod 3: 20 mm"));
    }

    #[test]
    fn test_ascii_correct_cut_widths() {
        let out = ascii_render(&[30, 20], &[100], &[vec![30, 20]]);

        assert!(out.contains("###############")); // 15
        assert!(out.contains("##########")); // 10
    }

    #[test]
    fn test_ascii_skips_empty_rods() {
        let out = ascii_render(&[50], &[100, 100], &[vec![50], vec![]]);

        assert!(out.contains("Rod 1: 100 mm"));
        assert!(!out.contains("Rod 2: 100 mm"));
    }

    #[test]
    fn test_ascii_cut_order_displayed_correctly() {
        let out = ascii_render(
            &[90, 60, 45],
            &[100, 100, 100],
            &[vec![90], vec![60], vec![45]],
        );

        assert!(out.contains("Cut  1:   90 mm"));
        assert!(out.contains("Cut  2:   60 mm"));
        assert!(out.contains("Cut  3:   45 mm"));
    }
}
