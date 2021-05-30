use criterion::{black_box, BenchmarkId, criterion_group, criterion_main, Criterion};
use graphql_parser::query::{parse_query, Document};
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use std::hash::BuildHasherDefault;
use twox_hash::XxHash64;
use fasthash::{murmur3, Murmur3HasherExt};

const QUERY: &str = "query ($numTripPatterns: Int!, $from: Location!, $to: Location!, $dateTime: DateTime!, $arriveBy: Boolean!, $wheelchair: Boolean!, $modes: [Mode]!, $transportSubmodes: [TransportSubmodeFilter], $maxPreTransitWalkDistance: Float, $walkSpeed: Float, $minimumTransferTime: Int, $allowBikeRental: Boolean, $useFlex: Boolean, $banned: InputBanned, $whiteListed: InputWhiteListed) {
    trip(
      numTripPatterns: $numTripPatterns,
      from: $from,
      to: $to,
      dateTime: $dateTime,
      arriveBy: $arriveBy,
      wheelchair: $wheelchair,
      modes: $modes,
      transportSubmodes: $transportSubmodes,
      maxPreTransitWalkDistance: $maxPreTransitWalkDistance,
      walkSpeed: $walkSpeed,
      minimumTransferTime: $minimumTransferTime,
      allowBikeRental: $allowBikeRental,
      useFlex: $useFlex,
      banned: $banned,
      whiteListed: $whiteListed
    ) {
      tripPatterns {
        expectedStartTime
        expectedEndTime
        directDuration
        duration
        distance
        walkDistance
        legs {
          ...legFields
        }
      }
    }
  }
  
  fragment legFields on Leg {
    aimedEndTime
    aimedStartTime
    authority {
      ...authorityFields
    }
    distance
    directDuration
    duration
    expectedEndTime
    expectedStartTime
    fromEstimatedCall {
      ...estimatedCallFields
    }
    fromPlace {
      ...placeFields
    }
    interchangeFrom {
      ...interchangeFields
    }
    interchangeTo {
      ...interchangeFields
    }
    intermediateEstimatedCalls {
      ...estimatedCallFields
    }
    line {
      ...lineFields
    }
    mode
    operator {
      ...operatorFields
    }
    pointsOnLink {
      ...pointsOnLinkFields
    }
    realtime
    ride
    rentedBike
    serviceJourney {
      ...serviceJourneyFields
    }
    situations {
      ...situationFields
    }
    toEstimatedCall {
      ...estimatedCallFields
    }
    toPlace {
      ...placeFields
    }
    transportSubmode
  }
  
  fragment lineFields on Line {
    bookingArrangements {
      ...bookingArrangementFields
    }
    description
    flexibleLineType
    id
    name
    notices {
      ...noticeFields
    }
    publicCode
    transportMode
    transportSubmode
  }
  
  fragment bookingArrangementFields on BookingArrangement {
    bookingMethods
    bookingNote
    minimumBookingPeriod
    bookingContact {
      phone
      url
    }
  }
  
  fragment noticeFields on Notice {
    text
  }
  
  fragment placeFields on Place {
    name
    latitude
    longitude
    quay {
      ...quayFields
    }
    bikeRentalStation {
      ...bikeRentalStationFields
    }
  }
  
  fragment quayFields on Quay {
    id
    name
    description
    publicCode
    situations {
      ...situationFields
    }
    stopPlace {
      ...stopPlaceFields
    }
  }
  
  fragment situationFields on PtSituationElement {
    situationNumber
    summary {
      language
      value
    }
    description {
      language
      value
    }
    advice {
      language
      value
    }
    lines {
      ...lineFields
    }
    validityPeriod {
      startTime
      endTime
    }
    reportType
    infoLinks {
      uri
      label
    }
  }
  
  fragment stopPlaceFields on StopPlace {
    id
    description
    name
    latitude
    longitude
    tariffZones {
      id
    }
  }
  
  fragment bikeRentalStationFields on BikeRentalStation {
    id
    name
    networks
    bikesAvailable
    spacesAvailable
    longitude
    latitude
  }
  
  fragment authorityFields on Authority {
    id
    name
    url
  }
  
  fragment operatorFields on Operator {
    id
    name
    url
  }
  
  fragment serviceJourneyFields on ServiceJourney {
    id
    journeyPattern {
      line {
        ...lineFields
      }
      notices {
        ...noticeFields
      }
    }
    notices {
      ...noticeFields
    }
    publicCode
    privateCode
    transportSubmode
  }
  
  fragment interchangeFields on Interchange {
    guaranteed
    staySeated
    FromServiceJourney {
      id
    }
    ToServiceJourney {
      id
    }
  }
  
  fragment pointsOnLinkFields on PointsOnLink {
    points
    length
  }
  
  fragment estimatedCallFields on EstimatedCall {
    actualArrivalTime
    actualDepartureTime
    aimedArrivalTime
    aimedDepartureTime
    cancellation
    date
    destinationDisplay {
      frontText
    }
    expectedDepartureTime
    expectedArrivalTime
    forAlighting
    forBoarding
    notices {
      ...noticeFields
    }
    predictionInaccurate
    quay {
      ...quayFields
    }
    realtime
    requestStop
    serviceJourney {
      ...serviceJourneyFields
    }
    situations {
      ...situationFields
    }
  }";
type Cache = HashMap<
    &'static str,
    Document<'static, &'static str>,
    BuildHasherDefault<XxHash64>
>;

pub fn parse(query: &'static str) -> Document<'static, &'static str> {
    parse_query::<&str>(query).unwrap()
}

pub fn cached_parse(query: &'static str, cache: &mut Cache) -> Document<'static, &'static str> {
    match cache.entry(query) {
        Occupied(entry) => entry.get().clone(),
        Vacant(entry) => {
            let result = parse_query::<&str>(query).unwrap();
            entry.insert(result).clone()
        }
    }
}

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("GraphQL Parsing");
    let mut cache: Cache = Default::default();
    for i in [20u64, 21u64].iter() {
        group.bench_with_input(BenchmarkId::new("Cached", i), i, 
            |b, _| b.iter(|| cached_parse(black_box(QUERY), &mut cache)));
        group.bench_with_input(BenchmarkId::new("No cache", i), i, 
            |b, _| b.iter(|| parse(black_box(QUERY))));
    }
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);