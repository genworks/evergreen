// SPDX-FileCopyrightText: Copyright (C) 2026 Genworks International
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use std::process::Command;

/// FORMAT's ~& at the start of a control string is a fresh line relative to
/// the destination stream's column, not to the empty output of this call
/// (CLHS 22.3.1.5; GitHub #941).
#[test]
fn leading_fresh_line_follows_the_stream_column_across_tiers() {
    for tier in ["interp", "t0", "t1", "t2"] {
        let program = r#"
          (defun after-abc (thunk)
            (with-output-to-string (s) (write-string "abc" s) (funcall thunk s)))
          (assert (equal (format nil "abc~%def") (after-abc (lambda (s) (format s "~&def")))))
          (assert (equal (format nil "abc~%~%def") (after-abc (lambda (s) (format s "~2&def")))))
          (assert (equal (format nil "abcx~%def") (after-abc (lambda (s) (format s "x~&def")))))
          (assert (equal (format nil "abc~%def")
                         (after-abc (lambda (s) (funcall (formatter "~&def") s)))))
          (assert (equal (format nil "abc~%def")
                         (after-abc (lambda (s) (format (make-broadcast-stream s) "~&def")))))
          (assert (equal (format nil "abc~%def")
                         (with-output-to-string (s) (format s "abc~%") (format s "~&def"))))
          (assert (equal "def" (with-output-to-string (s) (format s "~&def"))))
          (assert (equal "def" (format nil "~&def")))
          (assert (equal (format nil "~%def") (format nil "~2&def")))
          (write-line "FRESH-LINE-PASS")
        "#;
        let output = Command::new(env!("CARGO_BIN_EXE_egcl"))
            .args(["--no-init", "--eval", program])
            .env("EGCL_FORCE_TIER", tier)
            .env_remove("EGCL_BACKEND")
            .env_remove("EGCL_DISABLE_T2")
            .output()
            .expect("run FORMAT ~& checks");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{tier}: {stdout}\n{stderr}");
        assert!(stdout.contains("FRESH-LINE-PASS"), "{tier}: {stdout}\n{stderr}");
    }
}
