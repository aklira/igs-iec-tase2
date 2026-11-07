# igs-iec-tase2
Industrial-grade IEC 60870-6 ICCP TASE.2 protocol stack implementation for SCADA and telecontrol

## Limits

- **Dual use is not supported.** An association has one role, client or server, as IEC 60870-6-503 §5.3.3.3 allows.
  The dual-use mode, in which one association carries both directions, is outside 1.0 (decision D-054 in `PROVENANCE.md`).
