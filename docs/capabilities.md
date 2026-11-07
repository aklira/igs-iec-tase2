# Capabilities of igs-iec-tase2 1.0

This document states what 1.0 implements: the blocks of IEC 60870-6-702, the operations of IEC 60870-6-503 that the client and
the server serve, the test that covers each row, and the known gaps. A test named in the matrix must exist in the workspace or
in the bench (`crates/igs-iec-tase2/tests/capabilities.rs` checks it).

## Blocks

| Block | Base | In 1.0 | Reason |
| --- | --- | --- | --- |
| Basic Services | m | yes | Mandatory; the scope of 1.0 (decision D-009). |
| Extended Conditions, Blocked Transfers, Information Message, SBO Device Control | o | no | Optional; left out by D-009, D-044. |
| Programs, Events, Accounts, Time Series | i | no | Left out by D-009. |

The association mode is one role per association. The dual-use mode (one association for both directions) is not supported
(decision D-054).

## Operations

The matrix covers the client (`igs-iec-tase2::TaseClient`, `igstase2-client`) and the server (`igs-iec-tase2::TaseServer`,
`igstase2-server`). The bench evidence of the client is in `docs/client-matrix.md`.

<!-- matrix:begin -->

| Operation | Clause | Client | Server | Test |
| --- | --- | --- | --- | --- |
| Associate with the table, and read the table identifier, the TASE.2 version and the supported features | 503 §7.2.2.2.2 | yes | yes | `the_association_is_established_with_a_matching_table_and_version`, `the_association_is_established_with_the_special_objects_of_the_server` |
| A table identifier that differs concludes the association | 503 §5.3.3.3 | yes | yes | `a_mismatching_table_identifier_makes_the_client_conclude`, `a_client_with_another_table_identifier_is_concluded` |
| A client without a table is refused | 503 §5.3.2, §7.2.2.2.3 | yes | yes | `an_unknown_client_is_declined_with_a_negative_response`, `a_client_without_a_table_is_refused_the_association` |
| The designated initiator refuses an association with an Initiate-ErrorPDU | 503 §7.2.2.2.1 | no | yes | `a_server_that_is_the_designated_initiator_refuses_with_an_initiate_error` |
| The release of an association disables its transfer sets and deselects its devices | 503 §7.2.2.3.3 | no | yes | `the_release_disables_the_transfer_sets_and_deselects_the_devices` |
| Get Data Value, answered per item | 503 §7.2.3.2 | yes | yes | `get_data_value_answers_each_item`, `get_data_value_answers_what_the_table_allows` |
| Set Data Value, answered per item, and only where the table permits | 503 §7.2.3.3 | yes | yes | `set_data_value_writes_what_the_table_allows_only`, `a_change_the_store_refuses_is_neither_kept_nor_reported` |
| Get Data Value Type | 503 §7.2.3.5 | yes | yes | `get_data_value_type_describes_a_readable_value_only` |
| Create Data Set, with its checks in the order of the standard | 503 §7.2.4.2 | yes | yes | `create_data_set_defines_a_data_set_of_readable_members`, `a_data_set_is_created_by_the_client_and_refused_when_it_exists` |
| The client may use the data set it creates | 503 §5.2.3.4 (decision D-055) | yes | yes | `a_data_set_the_client_created_is_visible_to_it_and_deletable_by_it` |
| Delete Data Set, checked name by name | 503 §7.2.4.3 | yes | yes | `a_data_set_is_deleted_when_the_table_allows_it_and_an_unknown_one_is_refused` |
| Get Data Set Element Values, and the members of a data set | 503 §7.2.4.4 | yes | yes | `a_data_set_is_read_written_and_deleted_through_the_table` |
| Set Data Set Element Values, one answer per member | 503 §7.2.4.5 | yes | yes | `set_data_set_element_values_writes_each_permitted_member_in_order`, `a_data_set_is_read_written_and_deleted_through_the_table` |
| Start Transfer, and the reports of each change | 503 §7.2.5.2, §7.2.5.4 | yes | yes | `a_change_of_a_monitored_value_is_reported_to_its_association`, `a_transfer_report_of_the_server_is_received_as_an_event` |
| Reports by exception carry the changed members only | 503 §5.3.10.2.2 | yes | yes | `a_report_by_exception_carries_the_changed_member_only` |
| Reports at each interval, and at the integrity check | 503 §5.3.10.2.3 | no | yes | `the_interval_expiry_reports_the_entire_data_set_at_each_interval`, `an_integrity_expiry_reports_the_entire_data_set_with_rbe` |
| Buffered changes are reported at the end of the buffer | 503 §7.2.5.4.2 | no | yes | `a_buffered_change_is_reported_at_the_end_of_the_buffer_with_the_last_value` |
| A critical report not acknowledged in time is a data failure | 503 §7.2.5.5.3 | no | yes | `a_critical_report_not_acknowledged_in_time_is_a_data_failure`, `a_negative_acknowledgement_is_a_data_failure` |
| A report that would take longer than its TLE is discarded | 503 §5.3.10.2.2 | no | yes | `a_report_that_would_take_longer_than_its_tle_is_discarded` |
| Stop Transfer ends the reports | 503 §7.2.5.3.3 | yes | yes | `a_stopped_transfer_set_sends_no_further_report` |
| A transfer set the server does not hold, or a data set the table does not show, is refused per item | 503 §7.2.5.2.3 | yes | yes | `a_transfer_set_that_the_server_does_not_hold_is_refused_per_item`, `a_zero_interval_is_refused_at_the_start_of_the_interval_condition` |
| The check-back identifier is returned for a selectable device only | 503 §5.2.3.8 | no | yes | `the_check_back_id_is_returned_for_a_selectable_device_only` |
| The MMS types of the points, with their bits and ranges | 802 §6.2 to §6.3, §7.1 | model | model | `the_state_of_a_point_is_bits_zero_and_one_and_keeps_its_quality`, `a_control_point_type_outside_its_range_is_refused_by_the_type_and_by_mms` |
| The protection events and their flags | 802 §6.2.3, §7.1.3 | model | model | `the_bits_of_a_single_event_are_at_the_positions_of_the_standard` |
| The transfer set conditions and features | 503 §8.2.8, §8.2.9 | model | model | `the_bits_of_the_ds_conditions_are_at_the_positions_of_the_standard`, `the_main_text_names_prevail_over_the_annexes` |
| Random data is read by every type without a panic | M6 | model | model | `random_data_is_read_by_every_type_without_a_panic` |
| The demonstration file refuses an unknown key | `igstase2-server` | no | demo | `an_unknown_key_is_refused_with_its_line` |
| The bench scenarios of the client, one per operation | T3 | yes | no | `test_association_is_established_and_points_are_read` |

<!-- matrix:end -->

## Known gaps

Each gap is recorded with its decision; none is hidden.

- **Dual use** is not supported (D-054).
- **GetNameList** is not served by the server, and the client does not call it (D-049).
- **Blocked transfer reports** (`BlockData`) are refused per item by the server, and the client does not decode them (D-049).
- **The Data_failure event** is not sent by the server (D-049).
- **The acknowledgement of a critical report** is not sent by the client: the MMS client has no unconfirmed send (D-046).
- **The device operations** (Select, Operate, Set Tag) and the **information messages** are not in 1.0 (D-044).
- **Programs, Events, Accounts and Time Series** are not in 1.0 (D-009).
- **A point of an unsigned type** cannot be written by the command line, which sends integers and reals (D-056).
- **A transfer set is active only on the association that started it**: the command line cannot stop it in another command (D-056).
- **`TASE2_Version`** is spelled without the dot of the standard's text, which MMS identifiers do not allow without the `char`
  building block (D-045). Its confirmation with a real peer is open.
- **Interop with a third-party TASE.2 peer** is not done: the reference peer does not serve TASE.2 (D-010). The bench runs the
  client and the server of this repository.
- **Fuzzing** of the decoder for eight hours is not done (M6, set aside).
