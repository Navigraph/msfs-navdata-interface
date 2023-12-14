use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AirportCommunication {
    pub area_code: Option<String>,
    pub icao_code: Option<String>,
    pub airport_identifier: Option<String>,
    pub communication_type: Option<String>,
    pub communication_frequency: Option<f64>,
    pub frequency_units: Option<String>,
    pub service_indicator: Option<String>,
    pub callsign: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AirportMsa {
    pub area_code: Option<String>,
    pub icao_code: Option<String>,
    pub airport_identifier: Option<String>,
    pub msa_center: Option<String>,
    pub msa_center_latitude: Option<f64>,
    pub msa_center_longitude: Option<f64>,
    pub magnetic_true_indicator: Option<String>,
    pub multiple_code: Option<String>,
    pub radius_limit: Option<f64>,
    pub sector_bearing_1: Option<f64>,
    pub sector_altitude_1: Option<f64>,
    pub sector_bearing_2: Option<f64>,
    pub sector_altitude_2: Option<f64>,
    pub sector_bearing_3: Option<f64>,
    pub sector_altitude_3: Option<f64>,
    pub sector_bearing_4: Option<f64>,
    pub sector_altitude_4: Option<f64>,
    pub sector_bearing_5: Option<f64>,
    pub sector_altitude_5: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Airports {
    pub area_code: Option<String>,
    pub icao_code: String,
    pub airport_identifier: String,
    pub airport_identifier_3letter: Option<String>,
    pub airport_name: Option<String>,
    pub airport_ref_latitude: Option<f64>,
    pub airport_ref_longitude: Option<f64>,
    pub ifr_capability: Option<String>,
    pub longest_runway_surface_code: Option<String>,
    pub elevation: Option<f64>,
    pub transition_altitude: Option<f64>,
    pub transition_level: Option<f64>,
    pub speed_limit: Option<f64>,
    pub speed_limit_altitude: Option<f64>,
    pub iata_ata_designator: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlledAirspace {
    pub area_code: Option<String>,
    pub icao_code: Option<String>,
    pub airspace_center: Option<String>,
    pub controlled_airspace_name: Option<String>,
    pub airspace_type: Option<String>,
    pub airspace_classification: Option<String>,
    pub multiple_code: Option<String>,
    pub time_code: Option<String>,
    pub seqno: Option<f64>,
    pub flightlevel: Option<String>,
    pub boundary_via: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub arc_origin_latitude: Option<f64>,
    pub arc_origin_longitude: Option<f64>,
    pub arc_distance: Option<f64>,
    pub arc_bearing: Option<f64>,
    pub unit_indicator_lower_limit: Option<String>,
    pub lower_limit: Option<String>,
    pub unit_indicator_upper_limit: Option<String>,
    pub upper_limit: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CruisingTables {
    pub cruise_table_identifier: Option<String>,
    pub seqno: Option<f64>,
    pub course_from: Option<f64>,
    pub course_to: Option<f64>,
    pub mag_true: Option<String>,
    pub cruise_level_from1: Option<f64>,
    pub vertical_separation1: Option<f64>,
    pub cruise_level_to1: Option<f64>,
    pub cruise_level_from2: Option<f64>,
    pub vertical_separation2: Option<f64>,
    pub cruise_level_to2: Option<f64>,
    pub cruise_level_from3: Option<f64>,
    pub vertical_separation3: Option<f64>,
    pub cruise_level_to3: Option<f64>,
    pub cruise_level_from4: Option<f64>,
    pub vertical_separation4: Option<f64>,
    pub cruise_level_to4: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnrouteAirwayRestriction {
    pub area_code: Option<String>,
    pub route_identifier: Option<String>,
    pub restriction_identifier: Option<f64>,
    pub restriction_type: Option<String>,
    pub start_waypoint_identifier: Option<String>,
    pub start_waypoint_latitude: Option<f64>,
    pub start_waypoint_longitude: Option<f64>,
    pub end_waypoint_identifier: Option<String>,
    pub end_waypoint_latitude: Option<f64>,
    pub end_waypoint_longitude: Option<f64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub units_of_altitude: Option<String>,
    pub restriction_altitude1: Option<f64>,
    pub block_indicator1: Option<String>,
    pub restriction_altitude2: Option<f64>,
    pub block_indicator2: Option<String>,
    pub restriction_altitude3: Option<f64>,
    pub block_indicator3: Option<String>,
    pub restriction_altitude4: Option<f64>,
    pub block_indicator4: Option<String>,
    pub restriction_altitude5: Option<f64>,
    pub block_indicator5: Option<String>,
    pub restriction_altitude6: Option<f64>,
    pub block_indicator6: Option<String>,
    pub restriction_altitude7: Option<f64>,
    pub block_indicator7: Option<String>,
    pub restriction_notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnrouteAirways {
    pub area_code: Option<String>,
    pub route_identifier: Option<String>,
    pub seqno: Option<f64>,
    pub icao_code: Option<String>,
    pub waypoint_identifier: Option<String>,
    pub waypoint_latitude: Option<f64>,
    pub waypoint_longitude: Option<f64>,
    pub waypoint_description_code: Option<String>,
    pub route_type: Option<String>,
    pub flightlevel: Option<String>,
    pub direction_restriction: Option<String>,
    pub crusing_table_identifier: Option<String>,
    pub minimum_altitude1: Option<f64>,
    pub minimum_altitude2: Option<f64>,
    pub maximum_altitude: Option<f64>,
    pub outbound_course: Option<f64>,
    pub inbound_course: Option<f64>,
    pub inbound_distance: Option<f64>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnrouteCommunication {
    pub area_code: Option<String>,
    pub fir_rdo_ident: Option<String>,
    pub fir_uir_indicator: Option<String>,
    pub communication_type: Option<String>,
    pub communication_frequency: Option<f64>,
    pub frequency_units: Option<String>,
    pub service_indicator: Option<String>,
    pub remote_name: Option<String>,
    pub callsign: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnrouteNdbNavaids {
    pub area_code: Option<String>,
    pub icao_code: String,
    pub ndb_identifier: String,
    pub ndb_name: Option<String>,
    pub ndb_frequency: Option<f64>,
    pub navaid_class: Option<String>,
    pub ndb_latitude: Option<f64>,
    pub ndb_longitude: Option<f64>,
    pub range: Option<f64>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnrouteWaypoints {
    pub area_code: Option<String>,
    pub icao_code: String,
    pub waypoint_identifier: String,
    pub waypoint_name: Option<String>,
    pub waypoint_type: Option<String>,
    pub waypoint_usage: Option<String>,
    pub waypoint_latitude: Option<f64>,
    pub waypoint_longitude: Option<f64>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FirUir {
    pub area_code: Option<String>,
    pub fir_uir_identifier: Option<String>,
    pub fir_uir_address: Option<String>,
    pub fir_uir_name: Option<String>,
    pub fir_uir_indicator: Option<String>,
    pub seqno: Option<f64>,
    pub boundary_via: Option<String>,
    pub adjacent_fir_identifier: Option<String>,
    pub adjacent_uir_identifier: Option<String>,
    pub reporting_units_speed: Option<f64>,
    pub reporting_units_altitude: Option<f64>,
    pub fir_uir_latitude: Option<f64>,
    pub fir_uir_longitude: Option<f64>,
    pub arc_origin_latitude: Option<f64>,
    pub arc_origin_longitude: Option<f64>,
    pub arc_distance: Option<f64>,
    pub arc_bearing: Option<f64>,
    pub fir_upper_limit: Option<String>,
    pub uir_lower_limit: Option<String>,
    pub uir_upper_limit: Option<String>,
    pub cruise_table_identifier: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gate {
    pub area_code: Option<String>,
    pub airport_identifier: Option<String>,
    pub icao_code: Option<String>,
    pub gate_identifier: Option<String>,
    pub gate_latitude: Option<f64>,
    pub gate_longitude: Option<f64>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gls {
    pub area_code: Option<String>,
    pub airport_identifier: Option<String>,
    pub icao_code: Option<String>,
    pub gls_ref_path_identifier: Option<String>,
    pub gls_category: Option<String>,
    pub gls_channel: Option<f64>,
    pub runway_identifier: Option<String>,
    pub gls_approach_bearing: Option<f64>,
    pub station_latitude: Option<f64>,
    pub station_longitude: Option<f64>,
    pub gls_station_ident: Option<String>,
    pub gls_approach_slope: Option<f64>,
    pub magnetic_variation: Option<f64>,
    pub station_elevation: Option<f64>,
    pub station_type: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GridMora {
    pub starting_latitude: Option<f64>,
    pub starting_longitude: Option<f64>,
    pub mora01: Option<String>,
    pub mora02: Option<String>,
    pub mora03: Option<String>,
    pub mora04: Option<String>,
    pub mora05: Option<String>,
    pub mora06: Option<String>,
    pub mora07: Option<String>,
    pub mora08: Option<String>,
    pub mora09: Option<String>,
    pub mora10: Option<String>,
    pub mora11: Option<String>,
    pub mora12: Option<String>,
    pub mora13: Option<String>,
    pub mora14: Option<String>,
    pub mora15: Option<String>,
    pub mora16: Option<String>,
    pub mora17: Option<String>,
    pub mora18: Option<String>,
    pub mora19: Option<String>,
    pub mora20: Option<String>,
    pub mora21: Option<String>,
    pub mora22: Option<String>,
    pub mora23: Option<String>,
    pub mora24: Option<String>,
    pub mora25: Option<String>,
    pub mora26: Option<String>,
    pub mora27: Option<String>,
    pub mora28: Option<String>,
    pub mora29: Option<String>,
    pub mora30: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub version: String,
    pub arincversion: String,
    pub record_set: String,
    pub current_airac: String,
    pub revision: String,
    pub effective_fromto: String,
    pub previous_airac: String,
    pub previous_fromto: String,
    pub parsed_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Holdings {
    pub area_code: Option<String>,
    pub region_code: Option<String>,
    pub icao_code: Option<String>,
    pub waypoint_identifier: Option<String>,
    pub holding_name: Option<String>,
    pub waypoint_latitude: Option<f64>,
    pub waypoint_longitude: Option<f64>,
    pub duplicate_identifier: Option<f64>,
    pub inbound_holding_course: Option<f64>,
    pub turn_direction: Option<String>,
    pub leg_length: Option<f64>,
    pub leg_time: Option<f64>,
    pub minimum_altitude: Option<f64>,
    pub maximum_altitude: Option<f64>,
    pub holding_speed: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Iaps {
    pub area_code: Option<String>,
    pub airport_identifier: Option<String>,
    pub procedure_identifier: Option<String>,
    pub route_type: Option<String>,
    pub transition_identifier: Option<String>,
    pub seqno: Option<f64>,
    pub waypoint_icao_code: Option<String>,
    pub waypoint_identifier: Option<String>,
    pub waypoint_latitude: Option<f64>,
    pub waypoint_longitude: Option<f64>,
    pub waypoint_description_code: Option<String>,
    pub turn_direction: Option<String>,
    pub rnp: Option<f64>,
    pub path_termination: Option<String>,
    pub recommanded_navaid: Option<String>,
    pub recommanded_navaid_latitude: Option<f64>,
    pub recommanded_navaid_longitude: Option<f64>,
    pub arc_radius: Option<f64>,
    pub theta: Option<f64>,
    pub rho: Option<f64>,
    pub magnetic_course: Option<f64>,
    pub route_distance_holding_distance_time: Option<f64>,
    pub distance_time: Option<String>,
    pub altitude_description: Option<String>,
    pub altitude1: Option<f64>,
    pub altitude2: Option<f64>,
    pub transition_altitude: Option<f64>,
    pub speed_limit_description: Option<String>,
    pub speed_limit: Option<f64>,
    pub vertical_angle: Option<f64>,
    pub center_waypoint: Option<String>,
    pub center_waypoint_latitude: Option<f64>,
    pub center_waypoint_longitude: Option<f64>,
    pub aircraft_category: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub recommanded_id: Option<String>,
    #[serde(skip_serializing)]
    pub center_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalizerMarker {
    pub area_code: String,
    pub icao_code: String,
    pub airport_identifier: String,
    pub runway_identifier: String,
    pub llz_identifier: String,
    pub marker_identifier: String,
    pub marker_type: String,
    pub marker_latitude: f64,
    pub marker_longitude: f64,
    #[serde(skip_serializing)]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalizersGlideslopes {
    pub area_code: Option<String>,
    pub icao_code: Option<String>,
    pub airport_identifier: String,
    pub runway_identifier: Option<String>,
    pub llz_identifier: String,
    pub llz_latitude: Option<f64>,
    pub llz_longitude: Option<f64>,
    pub llz_frequency: Option<f64>,
    pub llz_bearing: Option<f64>,
    pub llz_width: Option<f64>,
    pub ils_mls_gls_category: Option<String>,
    pub gs_latitude: Option<f64>,
    pub gs_longitude: Option<f64>,
    pub gs_angle: Option<f64>,
    pub gs_elevation: Option<f64>,
    pub station_declination: Option<f64>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pathpoints {
    pub area_code: Option<String>,
    pub airport_identifier: Option<String>,
    pub icao_code: Option<String>,
    pub approach_procedure_ident: Option<String>,
    pub runway_identifier: Option<String>,
    pub sbas_service_provider_identifier: Option<f64>,
    pub reference_path_identifier: Option<String>,
    pub landing_threshold_latitude: Option<f64>,
    pub landing_threshold_longitude: Option<f64>,
    pub ltp_ellipsoid_height: Option<f64>,
    pub glidepath_angle: Option<f64>,
    pub flightpath_alignment_latitude: Option<f64>,
    pub flightpath_alignment_longitude: Option<f64>,
    pub course_width_at_threshold: Option<f64>,
    pub length_offset: Option<f64>,
    pub path_point_tch: Option<f64>,
    pub tch_units_indicator: Option<String>,
    pub hal: Option<f64>,
    pub val: Option<f64>,
    pub fpap_ellipsoid_height: Option<f64>,
    pub fpap_orthometric_height: Option<f64>,
    pub ltp_orthometric_height: Option<f64>,
    pub approach_type_identifier: Option<String>,
    pub gnss_channel_number: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictiveAirspace {
    pub area_code: Option<String>,
    pub icao_code: Option<String>,
    pub restrictive_airspace_designation: Option<String>,
    pub restrictive_airspace_name: Option<String>,
    pub restrictive_type: Option<String>,
    pub multiple_code: Option<String>,
    pub seqno: Option<f64>,
    pub boundary_via: Option<String>,
    pub flightlevel: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub arc_origin_latitude: Option<f64>,
    pub arc_origin_longitude: Option<f64>,
    pub arc_distance: Option<f64>,
    pub arc_bearing: Option<f64>,
    pub unit_indicator_lower_limit: Option<String>,
    pub lower_limit: Option<String>,
    pub unit_indicator_upper_limit: Option<String>,
    pub upper_limit: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Runways {
    pub area_code: Option<String>,
    pub icao_code: Option<String>,
    pub airport_identifier: String,
    pub runway_identifier: String,
    pub runway_latitude: Option<f64>,
    pub runway_longitude: Option<f64>,
    pub runway_gradient: Option<f64>,
    pub runway_magnetic_bearing: Option<f64>,
    pub runway_true_bearing: Option<f64>,
    pub landing_threshold_elevation: Option<f64>,
    pub displaced_threshold_distance: Option<f64>,
    pub threshold_crossing_height: Option<f64>,
    pub runway_length: Option<f64>,
    pub runway_width: Option<f64>,
    pub llz_identifier: Option<String>,
    pub llz_mls_gls_category: Option<String>,
    pub surface_code: Option<f64>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sids {
    pub area_code: Option<String>,
    pub airport_identifier: Option<String>,
    pub procedure_identifier: Option<String>,
    pub route_type: Option<String>,
    pub transition_identifier: Option<String>,
    pub seqno: Option<f64>,
    pub waypoint_icao_code: Option<String>,
    pub waypoint_identifier: Option<String>,
    pub waypoint_latitude: Option<f64>,
    pub waypoint_longitude: Option<f64>,
    pub waypoint_description_code: Option<String>,
    pub turn_direction: Option<String>,
    pub rnp: Option<f64>,
    pub path_termination: Option<String>,
    pub recommanded_navaid: Option<String>,
    pub recommanded_navaid_latitude: Option<f64>,
    pub recommanded_navaid_longitude: Option<f64>,
    pub arc_radius: Option<f64>,
    pub theta: Option<f64>,
    pub rho: Option<f64>,
    pub magnetic_course: Option<f64>,
    pub route_distance_holding_distance_time: Option<f64>,
    pub distance_time: Option<String>,
    pub altitude_description: Option<String>,
    pub altitude1: Option<f64>,
    pub altitude2: Option<f64>,
    pub transition_altitude: Option<f64>,
    pub speed_limit_description: Option<String>,
    pub speed_limit: Option<f64>,
    pub vertical_angle: Option<f64>,
    pub center_waypoint: Option<String>,
    pub center_waypoint_latitude: Option<f64>,
    pub center_waypoint_longitude: Option<f64>,
    pub aircraft_category: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub recommanded_id: Option<String>,
    #[serde(skip_serializing)]
    pub center_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stars {
    pub area_code: Option<String>,
    pub airport_identifier: Option<String>,
    pub procedure_identifier: Option<String>,
    pub route_type: Option<String>,
    pub transition_identifier: Option<String>,
    pub seqno: Option<f64>,
    pub waypoint_icao_code: Option<String>,
    pub waypoint_identifier: Option<String>,
    pub waypoint_latitude: Option<f64>,
    pub waypoint_longitude: Option<f64>,
    pub waypoint_description_code: Option<String>,
    pub turn_direction: Option<String>,
    pub rnp: Option<f64>,
    pub path_termination: Option<String>,
    pub recommanded_navaid: Option<String>,
    pub recommanded_navaid_latitude: Option<f64>,
    pub recommanded_navaid_longitude: Option<f64>,
    pub arc_radius: Option<f64>,
    pub theta: Option<f64>,
    pub rho: Option<f64>,
    pub magnetic_course: Option<f64>,
    pub route_distance_holding_distance_time: Option<f64>,
    pub distance_time: Option<String>,
    pub altitude_description: Option<String>,
    pub altitude1: Option<f64>,
    pub altitude2: Option<f64>,
    pub transition_altitude: Option<f64>,
    pub speed_limit_description: Option<String>,
    pub speed_limit: Option<f64>,
    pub vertical_angle: Option<f64>,
    pub center_waypoint: Option<String>,
    pub center_waypoint_latitude: Option<f64>,
    pub center_waypoint_longitude: Option<f64>,
    pub aircraft_category: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub recommanded_id: Option<String>,
    #[serde(skip_serializing)]
    pub center_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TerminalNdbNavaids {
    pub area_code: Option<String>,
    pub airport_identifier: String,
    pub icao_code: String,
    pub ndb_identifier: String,
    pub ndb_name: Option<String>,
    pub ndb_frequency: Option<f64>,
    pub navaid_class: Option<String>,
    pub ndb_latitude: Option<f64>,
    pub ndb_longitude: Option<f64>,
    pub range: Option<f64>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TerminalWaypoints {
    pub area_code: Option<String>,
    pub region_code: String,
    pub icao_code: String,
    pub waypoint_identifier: String,
    pub waypoint_name: Option<String>,
    pub waypoint_type: Option<String>,
    pub waypoint_latitude: Option<f64>,
    pub waypoint_longitude: Option<f64>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VhfNavaids {
    pub area_code: Option<String>,
    pub airport_identifier: Option<String>,
    pub icao_code: String,
    pub vor_identifier: String,
    pub vor_name: Option<String>,
    pub vor_frequency: Option<f64>,
    pub navaid_class: Option<String>,
    pub vor_latitude: Option<f64>,
    pub vor_longitude: Option<f64>,
    pub dme_ident: Option<String>,
    pub dme_latitude: Option<f64>,
    pub dme_longitude: Option<f64>,
    pub dme_elevation: Option<f64>,
    pub ilsdme_bias: Option<f64>,
    pub range: Option<f64>,
    pub station_declination: Option<f64>,
    pub magnetic_variation: Option<f64>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
}