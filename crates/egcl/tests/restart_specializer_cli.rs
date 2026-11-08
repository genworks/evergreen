// SPDX-FileCopyrightText: Copyright (C) 2026 Genworks International
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use std::process::Command;

/// RESTART is a standard class from the start: DEFMETHOD can specialize on it
/// before anything has called FIND-CLASS or made a restart (GitHub #917).
#[test]
fn defmethod_specializes_on_restart_first_across_tiers() {
    for tier in ["interp", "t0", "t1", "t2"] {
        let program = r#"
          (defgeneric restart-kind (x))
          (defmethod restart-kind ((x restart)) :restart)
          (defmethod restart-kind ((x t)) :other)
          (assert (eq :other (restart-kind 1)))
          (assert (eq :restart
                      (restart-case (restart-kind (find-restart 'probe))
                        (probe () nil))))
          (assert (eq (find-class 'restart) (find-class 'restart)))
          (write-line "RESTART-SPECIALIZER-PASS")
        "#;
        let output = Command::new(env!("CARGO_BIN_EXE_egcl"))
            .args(["--no-init", "--eval", program])
            .env("EGCL_FORCE_TIER", tier)
            .env_remove("EGCL_BACKEND")
            .env_remove("EGCL_DISABLE_T2")
            .output()
            .expect("run RESTART specializer checks");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{tier}: {stdout}\n{stderr}");
        assert!(
            stdout.contains("RESTART-SPECIALIZER-PASS"),
            "{tier}: {stdout}\n{stderr}"
        );
    }
}
