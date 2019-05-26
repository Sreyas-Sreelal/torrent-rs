use crate::proxy::proxy_setting;
use crate::torrent::{session_status, torrent_status};

#[test]
fn bindgen_test_layout_torrent_status() {
    assert_eq!(
        ::std::mem::size_of::<torrent_status>(),
        1752usize,
        concat!("Size of: ", stringify!(torrent_status))
    );
    assert_eq!(
        ::std::mem::align_of::<torrent_status>(),
        8usize,
        concat!("Alignment of ", stringify!(torrent_status))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).paused as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(paused)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).progress as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(progress)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).error as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).next_announce as *const _ as usize },
        1036usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(next_announce)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<torrent_status>())).announce_interval as *const _ as usize
        },
        1040usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(announce_interval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).current_tracker as *const _ as usize },
        1044usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(current_tracker)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).total_download as *const _ as usize },
        1560usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(total_download)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).total_upload as *const _ as usize },
        1568usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(total_upload)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<torrent_status>())).total_payload_download as *const _ as usize
        },
        1576usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(total_payload_download)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<torrent_status>())).total_payload_upload as *const _ as usize
        },
        1584usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(total_payload_upload)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<torrent_status>())).total_failed_bytes as *const _ as usize
        },
        1592usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(total_failed_bytes)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<torrent_status>())).total_redundant_bytes as *const _ as usize
        },
        1600usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(total_redundant_bytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).download_rate as *const _ as usize },
        1608usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(download_rate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).upload_rate as *const _ as usize },
        1612usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(upload_rate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<torrent_status>())).download_payload_rate as *const _ as usize
        },
        1616usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(download_payload_rate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<torrent_status>())).upload_payload_rate as *const _ as usize
        },
        1620usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(upload_payload_rate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).num_seeds as *const _ as usize },
        1624usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(num_seeds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).num_peers as *const _ as usize },
        1628usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(num_peers)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).num_complete as *const _ as usize },
        1632usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(num_complete)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).num_incomplete as *const _ as usize },
        1636usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(num_incomplete)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).list_seeds as *const _ as usize },
        1640usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(list_seeds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).list_peers as *const _ as usize },
        1644usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(list_peers)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<torrent_status>())).connect_candidates as *const _ as usize
        },
        1648usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(connect_candidates)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).num_pieces as *const _ as usize },
        1652usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(num_pieces)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).total_done as *const _ as usize },
        1656usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(total_done)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<torrent_status>())).total_wanted_done as *const _ as usize
        },
        1664usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(total_wanted_done)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).total_wanted as *const _ as usize },
        1672usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(total_wanted)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<torrent_status>())).distributed_copies as *const _ as usize
        },
        1680usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(distributed_copies)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).block_size as *const _ as usize },
        1684usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(block_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).num_uploads as *const _ as usize },
        1688usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(num_uploads)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).num_connections as *const _ as usize },
        1692usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(num_connections)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).uploads_limit as *const _ as usize },
        1696usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(uploads_limit)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<torrent_status>())).connections_limit as *const _ as usize
        },
        1700usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(connections_limit)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<torrent_status>())).up_bandwidth_queue as *const _ as usize
        },
        1704usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(up_bandwidth_queue)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<torrent_status>())).down_bandwidth_queue as *const _ as usize
        },
        1708usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(down_bandwidth_queue)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).all_time_upload as *const _ as usize },
        1712usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(all_time_upload)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<torrent_status>())).all_time_download as *const _ as usize
        },
        1720usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(all_time_download)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).active_time as *const _ as usize },
        1728usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(active_time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).seeding_time as *const _ as usize },
        1732usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(seeding_time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).seed_rank as *const _ as usize },
        1736usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(seed_rank)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).last_scrape as *const _ as usize },
        1740usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(last_scrape)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).has_incoming as *const _ as usize },
        1744usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(has_incoming)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<torrent_status>())).seed_mode as *const _ as usize },
        1748usize,
        concat!(
            "Offset of field: ",
            stringify!(torrent_status),
            "::",
            stringify!(seed_mode)
        )
    );
}

#[test]
fn bindgen_test_layout_session_status() {
    assert_eq!(
        ::std::mem::size_of::<session_status>(),
        196usize,
        concat!("Size of: ", stringify!(session_status))
    );
    assert_eq!(
        ::std::mem::align_of::<session_status>(),
        8usize,
        concat!("Alignment of ", stringify!(session_status))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).has_incoming_connections as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(has_incoming_connections)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<session_status>())).upload_rate as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(upload_rate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<session_status>())).download_rate as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(download_rate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<session_status>())).total_download as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(total_download)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<session_status>())).total_upload as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(total_upload)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).payload_upload_rate as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(payload_upload_rate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).payload_download_rate as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(payload_download_rate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).total_payload_download as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(total_payload_download)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).total_payload_upload as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(total_payload_upload)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).ip_overhead_upload_rate as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(ip_overhead_upload_rate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).ip_overhead_download_rate as *const _
                as usize
        },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(ip_overhead_download_rate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).total_ip_overhead_download as *const _
                as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(total_ip_overhead_download)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).total_ip_overhead_upload as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(total_ip_overhead_upload)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<session_status>())).dht_upload_rate as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(dht_upload_rate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).dht_download_rate as *const _ as usize
        },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(dht_download_rate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).total_dht_download as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(total_dht_download)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<session_status>())).total_dht_upload as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(total_dht_upload)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).tracker_upload_rate as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(tracker_upload_rate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).tracker_download_rate as *const _ as usize
        },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(tracker_download_rate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).total_tracker_download as *const _ as usize
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(total_tracker_download)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).total_tracker_upload as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(total_tracker_upload)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).total_redundant_bytes as *const _ as usize
        },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(total_redundant_bytes)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).total_failed_bytes as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(total_failed_bytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<session_status>())).num_peers as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(num_peers)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<session_status>())).num_unchoked as *const _ as usize },
        148usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(num_unchoked)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).allowed_upload_slots as *const _ as usize
        },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(allowed_upload_slots)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).up_bandwidth_queue as *const _ as usize
        },
        156usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(up_bandwidth_queue)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).down_bandwidth_queue as *const _ as usize
        },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(down_bandwidth_queue)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).up_bandwidth_bytes_queue as *const _ as usize
        },
        164usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(up_bandwidth_bytes_queue)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).down_bandwidth_bytes_queue as *const _
                as usize
        },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(down_bandwidth_bytes_queue)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<session_status>())).optimistic_unchoke_counter as *const _
                as usize
        },
        172usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(optimistic_unchoke_counter)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<session_status>())).unchoke_counter as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(unchoke_counter)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<session_status>())).dht_nodes as *const _ as usize },
        180usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(dht_nodes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<session_status>())).dht_node_cache as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(dht_node_cache)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<session_status>())).dht_torrents as *const _ as usize },
        188usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(dht_torrents)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<session_status>())).dht_global_nodes as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(session_status),
            "::",
            stringify!(dht_global_nodes)
        )
    );
}

#[test]
fn bindgen_test_layout_proxy_setting() {
    assert_eq!(
        ::std::mem::size_of::<proxy_setting>(),
        776usize,
        concat!("Size of: ", stringify!(proxy_setting))
    );
    assert_eq!(
        ::std::mem::align_of::<proxy_setting>(),
        4usize,
        concat!("Alignment of ", stringify!(proxy_setting))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<proxy_setting>())).hostname as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(proxy_setting),
            "::",
            stringify!(hostname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<proxy_setting>())).port as *const _ as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(proxy_setting),
            "::",
            stringify!(port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<proxy_setting>())).username as *const _ as usize },
        260usize,
        concat!(
            "Offset of field: ",
            stringify!(proxy_setting),
            "::",
            stringify!(username)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<proxy_setting>())).password as *const _ as usize },
        516usize,
        concat!(
            "Offset of field: ",
            stringify!(proxy_setting),
            "::",
            stringify!(password)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<proxy_setting>())).type_ as *const _ as usize },
        772usize,
        concat!(
            "Offset of field: ",
            stringify!(proxy_setting),
            "::",
            stringify!(type_)
        )
    );
}
