// SPDX-FileCopyrightText: Copyright (C) 2026 Genworks International
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use std::process::Command;

/// A `#` given constituent syntax is no longer a dispatching macro character;
/// rebuilt with MAKE-DISPATCH-MACRO-CHARACTER, it reads the standard
/// sub-characters restored into it, with the current readtable (GitHub #940).
#[test]
fn rebuilt_sharp_reads_restored_standard_sub_characters_across_tiers() {
    for tier in ["interp", "t0", "t1", "t2"] {
        let program = r##"
          (defun std-sub (sub) (get-dispatch-macro-character #\# sub (copy-readtable nil)))
          (let ((rt (copy-readtable nil)))
            (set-syntax-from-char #\# #\A rt)
            (assert (handler-case (progn (get-dispatch-macro-character #\# #\' rt) nil)
                      (error () t)))
            (make-dispatch-macro-character #\# t rt)
            (dolist (sub '(#\' #\( #\: #\\ #\+))
              (set-dispatch-macro-character #\# sub (std-sub sub) rt))
            (let ((*readtable* rt))
              (assert (equal '(function car) (read-from-string "#'car")))
              (assert (equalp #(1 2) (read-from-string "#(1 2)")))
              (assert (equal "FOO" (symbol-name (read-from-string "#:foo"))))
              (assert (null (symbol-package (read-from-string "#:foo"))))
              (assert (eql #\a (read-from-string "#\\a")))
              (assert (eq :x (read-from-string "#+common-lisp :x")))
              ;; A sub-character not restored stays undefined.
              (assert (handler-case (progn (read-from-string "#.(+ 1 2)") nil)
                        (error () t)))))
          (let ((rt (copy-readtable nil)))
            (setf (readtable-case rt) :invert)
            (make-dispatch-macro-character #\# t rt)
            (set-dispatch-macro-character #\# #\: (std-sub #\:) rt)
            (let ((*readtable* rt))
              (assert (equal "foo" (symbol-name (read-from-string "#:FOO"))))))
          (assert (equal '(function car) (read-from-string "#'car")))
          (write-line "SHARP-REDISPATCH-PASS")
        "##;
        let output = Command::new(env!("CARGO_BIN_EXE_egcl"))
            .args(["--no-init", "--eval", program])
            .env("EGCL_FORCE_TIER", tier)
            .env_remove("EGCL_BACKEND")
            .env_remove("EGCL_DISABLE_T2")
            .output()
            .expect("run # redispatch checks");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{tier}: {stdout}\n{stderr}");
        assert!(
            stdout.contains("SHARP-REDISPATCH-PASS"),
            "{tier}: {stdout}\n{stderr}"
        );
    }
}
