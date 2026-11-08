# Client interop matrix (T3)

The client under test is `igstase2-client` (`docs/igstase2-client.md`), run in the bench's client service (`client-tase2`).
The device under test is `igstase2-server` (`docs/igstase2-server.md`), in the bench's device service (`device-tase2`), with
`bench/device/tase2-demo.toml`. The scenarios are `bench/tests/test_t3_client_matrix.py`: one test per row, named in
backticks. A test that names no row, or a row that names no existing test, fails.

The reference peer of the plan (rusty-iccp) does not serve TASE.2 (decision D-010), so this matrix is run against our own
device. It is not a third-party interop result.

## Scenarios

| Operation | Scenario | Test | Result |
| --- | --- | --- | --- |
| Association | The client associates with the table of its AP-title, reads the points | `test_association_is_established_and_points_are_read` | pass |
| Association | An AP-title without a table is refused (status 1) | `test_an_ap_title_without_a_table_is_refused_with_status_1` | pass |
| Association | A table identifier that differs is concluded (status 1) | `test_a_table_identifier_that_differs_is_concluded_with_status_1` | pass |
| Get Data Value | A point that does not exist is an item error | `test_a_point_that_does_not_exist_is_an_item_error` | pass |
| Set Data Value | A permitted write is applied and read back | `test_a_permitted_write_is_applied_and_read_back` | pass |
| Set Data Value | A write the table does not permit is refused per item | `test_a_write_the_table_does_not_permit_is_refused_per_item` | pass |
| Get Data Value Type | The type of a point is described | `test_the_type_of_a_point_is_described` | pass |
| Get Data Set Names | The members of a data set are listed | `test_the_members_of_a_data_set_are_listed` | pass |
| Get Data Set Element Values | A data set is read member by member | `test_a_data_set_is_read_member_by_member` | pass |
| Set Data Set Element Values | A data set write answers each member | `test_a_data_set_write_answers_each_member` | pass |
| Create / Delete Data Set | A data set the client creates is readable and deletable by it | `test_a_data_set_the_client_creates_can_be_read_and_deleted` | pass |
| Data set not shown | A data set the table does not show is refused (status 1) | `test_a_data_set_the_table_does_not_show_is_refused_with_status_1` | pass |
| Delete Data Set | A data set that does not exist is refused (status 1) | `test_a_data_set_that_does_not_exist_cannot_be_deleted` | pass |
| Start Transfer | Each change of the data set is reported | `test_a_transfer_set_reports_each_change_of_its_data_set` | pass |
| Start Transfer | An interval transfer set reports at its interval | `test_an_interval_transfer_set_reports_at_its_interval` | pass |
| Start Transfer | A report by exception names the changed members | `test_a_report_by_exception_names_only_the_changed_members` | pass |
| Stop Transfer | A transfer set that is not active on the association is refused | `test_a_stop_of_a_transfer_set_that_is_not_active_is_refused` | pass |
| Start Transfer | A transfer set on a data set that does not exist is refused | `test_a_transfer_set_on_a_data_set_that_does_not_exist_is_refused` | pass |
| Select | A Select returns the CheckBackID of the table, and arms the device | `test_a_select_returns_the_check_back_id_of_the_table` | pass |
| Select | A device armed for one client refuses another, and is released at its timeout | `test_a_device_armed_for_one_client_refuses_another_select_and_releases_at_its_timeout` | pass |
| Operate | An Operate selects the SBO device, then operates it, and the device is IDLE afterwards | `test_an_operate_selects_then_operates_the_sbo_device` | pass |
| Operate | An Operate of an SBO device that is not selected is refused | `test_an_operate_of_an_sbo_device_without_its_select_is_refused` | pass |
| Operate | A closing command is refused under CLOSE-ONLY-INHIBIT, another command is not | `test_a_close_only_inhibit_tag_refuses_the_closing_command_and_not_another` | pass |
| Set Tag / Get Tag | A tag and its reason are set and read, and an OPEN-AND-CLOSE-INHIBIT tag refuses the Select | `test_a_tag_and_its_reason_are_read_and_inhibit_the_select` | pass |
| Start Information Transfer | The information messages of the server are reported, in order | `test_an_information_message_is_reported_to_the_client` | pass |
| Stop Information Transfer | The stop is accepted (its effect is tested by the server's tests, see the gaps) | `test_the_stop_of_the_information_transfer_set_is_accepted` | pass |
| Start Transfer (block 2) | The integrity check reports the entire data set, with RBE, at its interval | `test_an_integrity_check_reports_the_entire_data_set_with_rbe` | pass |
| Start Transfer (block 2) | A buffered change is reported, with a TLE | `test_a_buffered_object_change_is_reported_with_its_tle` | pass |
| Matrix | Every test named here exists in the bench | `test_every_test_the_client_matrix_names_exists_here` | pass |

## Gaps (rule 10): recorded, not tested

- **Dual use** (one association for both directions): not supported (decision D-054).
- **Acknowledgement of a critical report**: the client does not send it, because the MMS client has no unconfirmed send
  (decision D-046 of the client; the MMS stack's gap).
- **Blocked transfer reports** (`BlockData`): the server refuses them per item; the client does not decode them.
- **GetNameList**: the client calls it to list the variables and the data sets of its domain (`names`, every page of the list);
  the server does not serve it (decision D-049), so the listing is checked against the peer's server only.
- **Data_failure event**: not sent by the server (decision D-049).
- **Stop in the same association as the start**: the command line starts and reports in one association and concludes it, so
  the stop of an active transfer set cannot be shown in one command (decision D-056).
- **Writing an unsigned point**: the command line sends integers and reals; a point of an unsigned type refuses them (decision
  D-056).
- **Several associations at once, and the TLE discard**: not exercised by this matrix. The server's tests cover the isolation of
  the reports between two associations, and a TLE that is met; the discard of a late report is not provoked on the bench.
- **A non-SBO device** on the bench: the demonstration file names one control point, an SBO breaker. The server's tests cover the
  Direct-Control path (`a_non_sbo_device_is_operated_directly_and_cannot_be_selected`).
- **The stop of an information transfer set in the same association as its start**: the command line cannot do both in one
  association. The server's tests cover that no message follows the stop (`no_information_message_is_reported_before_the_start_or_after_the_stop`).
- **The commands of a device are simulated**: an accepted Operate completes at once, so the outcome of a real device, and its
  Failure (HARDWARE-FAULT), are not shown on the bench (decision D-066). The device actions are not sent as MMS events (D-066).
- **The Local Reset** is not an operation of the application on the server (D-066): not shown.
- **Set-point commands** are not on the bench: the command line writes an integer, a Boolean or a real, and checks no type against
  the control point (the server does not check it either).
- **Information messages sent by the client**: not in 1.0 (decision D-061).
