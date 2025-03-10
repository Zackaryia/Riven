#![cfg_attr(feature = "nightly", feature(custom_test_frameworks))]
#![cfg_attr(feature = "nightly", test_runner(my_runner))]

mod async_tests;
mod testutils;
use testutils::*;

use colored::*;

use riven::consts::*;

const ROUTE: PlatformRoute = PlatformRoute::JP1;

static MATCHES: &[&str] = &[
    // Regular game:
    "KR_5495121707",
    // `teamPosition` empty:
    // AFK:
    "JP1_312062554",
    "JP1_326464722",
    "JP1_289504387",
    "JP1_285434511",
    "JP1_307559381",
    "JP1_292569767",
    "JP1_310138781",
    "JP1_300507433",
    "JP1_283568774",
    // `individualPosition` is set but `teamPosition` is empty due to AFK slightly after beginning:
    "JP1_285797147",
    // Illegal big `championId`s. https://github.com/RiotGames/developer-relations/issues/553
    "JP1_267647303",
    "JP1_273343663",
    // Only has participant IDs for blue team.
    "JP1_391732436",
    // New field `ParticipantChallenges` `twoWardsOneSweeperCount`
    "JP1_397348569",
    // New fields:
    // `match-v5.ParticipantDto.playerAugment[1234],playerSubteamId,subteamPlacement`
    "JP1_400700181",
];

async_tests! {
    my_runner {
        // Summoner tests.
        summoner_get_kanjikana: async {
            let p = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, "私の 頭が かたい");
            let s = p.await.map_err(|e| e.to_string())?.ok_or_else(|| "Failed to get myheadhard".to_owned())?;
            rassert_eq!("私の頭がかたい", s.name);
            Ok(())
        },

        // Failure cases.
        // // Make sure get_raw_response(...) with invalid path fails as expected.
        // raw_response_invalid: async {
        //     let p = RIOT_API.get_raw_response("summoner-v4.getBySummonerName", Region::JP.into(), "INVALID/PATH".to_owned(), None);
        //     let r = p.await;
        //     rassert!(r.is_err());
        //     Ok(())
        // },
        // summoner_v4().get_by_summoner_name(...) normally returns an option.
        // If we use `get` (instead of `get_optional`) make sure it errors.
        get_nonoptional_invalid: async {
            let path_string = format!("/lol/summoner/v4/summoners/by-name/{}", "SUMMONER THAT DOES NOT EXIST");
            let request = RIOT_API.request(reqwest::Method::GET, ROUTE.into(), &path_string);
            let p = RIOT_API.execute_val::<riven::models::summoner_v4::Summoner>(
                "summoner-v4.getBySummonerName", ROUTE.into(), request);
            let r = p.await;
            rassert!(r.is_err());
            Ok(())
        },
        // Make sure 403 is handled as expected.
        tournament_forbidden: async {
            let p = RIOT_API.tournament_v4().get_tournament_code(ROUTE.to_regional(), "INVALID_CODE");
            let r = p.await;
            rassert!(r.is_err());
            rassert_eq!(Some(reqwest::StatusCode::FORBIDDEN), r.unwrap_err().status_code());
            Ok(())
        },

        // Disabled: Caihonbbt no longer ranked.
        // // tft-league-v1.getLeagueEntriesForSummoner
        // // https://github.com/MingweiSamuel/Riven/issues/25
        // tft_league_getleagueentriesforsummoner: async {
        //     let sp = RIOT_API.summoner_v4().get_by_summoner_name(ROUTE, "Caihonbbt");
        //     let sr = sp.await.map_err(|e| e.to_string())?.ok_or_else(|| "Failed to get \"Caihonbbt\"".to_owned())?;
        //     let lp = RIOT_API.tft_league_v1().get_league_entries_for_summoner(ROUTE, &sr.id);
        //     let lr = lp.await.map_err(|e| e.to_string())?;
        //     rassert!(!lr.is_empty());
        //     Ok(())
        // },
        // tft-league-v1.getTopRatedLadder
        // https://github.com/MingweiSamuel/Riven/issues/24
        tft_league_gettopratedladder: async {
            let lp = RIOT_API.tft_league_v1().get_top_rated_ladder(ROUTE, QueueType::RANKED_TFT_TURBO);
            let lr = lp.await.map_err(|e| e.to_string())?;
            rassert!(!lr.is_empty());
            Ok(())
        },

        // ASIA regional tests
        league_v4_match_v5_latest_combo: async {
            league_v4_match_v5_latest_combo(ROUTE).await
        },
        match_v5_get: async {
            match_v5_get(ROUTE.to_regional(), MATCHES).await
        },
        match_v5_get_timeline: async {
            match_v5_get_timeline(ROUTE.to_regional(), MATCHES).await
        },
    }
}
