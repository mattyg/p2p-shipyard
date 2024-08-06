#[cfg(all(feature = "gossip_arc_clamping_empty", feature = "gossip_arc_clamping_full"))]
compile_error!(
    "The `gossip_arc_clamping_empty` and `gossip_arc_clamping_full` features are both enabled, which is an error. Please enable only once."
);
#[cfg(all(feature = "gossip_arc_clamping_empty", feature = "gossip_arc_no_clamping"))]
compile_error!(
    "The `gossip_arc_clamping_empty` and `gossip_arc_no_clamping` features are both enabled, which is an error. Please enable only once."
);
#[cfg(all(feature = "gossip_arc_clamping_full", feature = "gossip_arc_no_clamping"))]
compile_error!(
    "The `gossip_arc_clamping_full` and `gossip_arc_no_clamping` features are both enabled, which is an error. Please enable only once."
);

#[cfg(all(not(feature = "gossip_arc_clamping_empty"), not(feature = "gossip_arc_clamping_full"), not(feature = "gossip_arc_no_clamping")))]
compile_error!("All of the `gossip_arc_clamping_empty`, `gossip_arc_clamping_full`, and `gossip_arc_no_clamping` features are disabled. Please enable one of them.");
