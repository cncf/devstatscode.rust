pub mod lib {
    use regex::Regex;
    use std::collections::{HashMap, HashSet};
    use std::time::{Duration, SystemTime};

    #[derive(Debug)]
    pub struct Ctx {
        pub data_dir: String,                // From GHA2DB_DATADIR, default /etc/gha2db/
        pub debug: u8, // From GHA2DB_DEBUG Debug level: 0-no, 1-info, 2-verbose, including SQLs, default 0
        pub cmd_debug: u8, // From GHA2DB_CMDDEBUG Commands execution Debug level: 0-no, 1-only output commands, 2-output commands and their output, 3-output full environment as well, default 0
        pub github_debug: u8, // From GHA2DB_GITHUB_DEBUG debug GitHub rate limits
        pub dry_run: bool, // From GHA2DB_DRY_RUN, import_affs tool - stop before doing any updates
        pub json_out: bool, // From GHA2DB_JSON gha2db: write JSON files? default false
        pub db_out: bool,  // From GHA2DB_NODB gha2db: write to SQL database, default true
        pub st: bool, // From GHA2DB_ST true: use single threaded version, false: use multi threaded version, default false
        pub ncpus: u16, // From GHA2DB_NCPUS, set to override number of CPUs to run, this overwrites GHA2DB_ST, default 0 (which means do not use it)
        pub pg_host: String, // From PG_HOST, default "localhost"
        pub pg_port: String, // From PG_PORT, default "5432"
        pub pg_db: String, // From PG_DB, default "gha"
        pub pg_user: String, // From PG_USER, default "gha_admin"
        pub pg_pass: String, // From PG_PASS, default "password"
        pub pg_ssl: String, // From PG_SSL, default "disable"
        pub index: bool, // From GHA2DB_INDEX Create DB index? default false
        pub table: bool, // From GHA2DB_SKIPTABLE Create table structure? default true
        pub tools: bool, // From GHA2DB_SKIPTOOLS Create DB tools (like views, summary tables, materialized views etc)? default true
        pub mgetc: String, // From GHA2DB_MGETC Character returned by mgetc (if non empty), default ""
        pub qout: bool,    // From GHA2DB_QOUT output all SQL queries?, default false
        pub ctx_out: bool, // From GHA2DB_CTXOUT output all context data (this struct), default false
        pub log_time: bool, // From GHA2DB_SKIPTIME, output time with all lib.Printf(...) calls, default true, use GHA2DB_SKIPTIME to disable
        pub default_start_date: SystemTime, // From GHA2DB_STARTDT, default `2012-07-01 00:00 UTC`, expects format "YYYY-MM-DD HH:MI:SS", can be set in `projects.yaml` via `start_date:`, value from projects.yaml (if set) has the highest priority.
        pub force_start_date: bool,         // From GHA2DB_STARTDT_FORCE, default false
        pub last_series: String, // From GHA2DB_LASTSERIES, use this TSDB series to determine last timestamp date, default "events_h"
        pub skip_tsdb: bool, // From GHA2DB_SKIPTSDB gha2db_sync tool, skip TS DB processing? for calc_metric it skips final series write, default false
        pub skip_pdb: bool, // From GHA2DB_SKIPPDB gha2db_sync tool, skip Postgres DB processing (gha2db part) default false
        pub reset_tsdb: bool, // From GHA2DB_RESETTSDB sync tool, regenerate all TS points? default false
        pub reset_ranges: bool, // From GHA2DB_RESETRANGES sync tool, regenerate all past quick ranges? default false
        pub explain: bool, // From GHA2DB_EXPLAIN runq tool, prefix query with "explain " - it will display query plan instead of executing real query, default false
        pub old_format: bool, // From GHA2DB_OLDFMT gha2db tool, if set then use pre 2015 GHA JSONs format
        pub exact: bool, // From GHA2DB_EXACT gha2db tool, if set then orgs list provided from commandline is used as a list of exact repository full names, like "a/b,c/d,e", if not only full names "a/b,x/y" can be treated like this, names without "/" are either orgs or repos.
        pub log_to_db: bool, // From GHA2DB_SKIPLOG all tools, if set, DB logging into Postgres table `gha_logs` in `devstats` database will be disabled
        pub local: bool, // From GHA2DB_LOCAL many tools, if set it will use data files prefixed with "./" to use local ones. Otherwise it will search for data files in /etc/gha2db.
        pub absolute: bool, // From GHA2DB_ABSOLUTE runq tool, if set it will use data files without any prefix (allowing absolute paths as well). Otherwise it will search for data files in /etc/gha2db.
        pub local_cmd: bool, // From GHA2DB_LOCAL_CMD many tools, if set it will call other tools prefixed with "./" to use locally compiled ones. Otherwise it will call binaries without prefix (so it will use those in $PATH).
        pub metrics_yaml: String, // From GHA2DB_METRICS_YAML gha2db_sync tool, set other metrics.yaml file, default is "metrics/{{project}}metrics.yaml"
        pub tags_yaml: String, // From GHA2DB_TAGS_YAML tags tool, set other tags.yaml file, default is "metrics/{{project}}/tags.yaml"
        pub columns_yaml: String, // From GHA2DB_COLUMNS_YAML tags tool, set other columns.yaml file, default is "metrics/{{project}}/columns.yaml"
        pub vars_yaml: String, // From GHA2DB_VARS_YAML db_vars tool, set other vars.yaml file (full path), default is "metrics/{{project}}/vars.yaml"
        pub vars_fn_yamlaml: String, // From GHA2DB_VARS_FN_YAML db_vars tool, set other vars.yaml file (final file name without path), default is "vars.yaml"
        pub skip_dates_yaml: String, // From GHA2DB_SKIP_DATES_YAML gha2db tool, set other skip_dates.yaml file, default is "skip_dates.yaml"
        pub github_oauth: String, // From GHA2DB_GITHUB_OAUTH ghapi2db tool, if not set reads from /etc/github/oauth file, set to "-" to force public access.
        pub clear_db_period: String, // From GHA2DB_MAXLOGAGE gha2db_sync tool, maximum age of devstats.gha_logs entries, default "1 week"
        pub clear_affs_lock_period: String, // From GHA2DB_MAX_AFFS_LOCK_AGE devstats tool, maximum age of devstats.gha_metrics "affs_lock" age, default "16 hours"
        pub clear_giant_lock_period: String, // From GHA2DB_MAX_GIANT_LOCK_AGE devstats tool, maximum age of devstats.gha_metrics "giant_lock" age, default "40 hours"
        pub trials: Vec<i16>, // From GHA2DB_TRIALS, all Postgres related tools, retry periods for some retryable errors
        pub webhook_root: String, // From GHA2DB_WHROOT, webhook tool, default "/hook", must match .travis.yml notifications webhooks
        pub webhook_port: String, // From GHA2DB_WHPORT, webhook tool, default ":1982", note that webhook listens using http:1982, but we use apache on https:2982 (to enable https protocol and proxy requests to http:1982)
        pub webhook_host: String, // From GHA2DB_WHHOST, webhook tool, default "127.0.0.1" (this can be localhost to disable access by IP, we use Apache proxy to enable https and then apache only need 127.0.0.1)
        pub check_payload: bool, // From GHA2DB_SKIP_VERIFY_PAYLOAD, webhook tool, default true, use GHA2DB_SKIP_VERIFY_PAYLOAD=1 to manually test payloads
        pub full_deploy: bool, // From GHA2DB_SKIP_FULL_DEPLOY, webhook tool, default true, use GHA2DB_SKIP_FULL_DEPLOY=1 to ignore "[deploy]" requests that call `./devel/deploy_all.sh`.
        pub deploy_branches: Vec<String>, // From GHA2DB_DEPLOY_BRANCHES, webhook tool, default "master" - comma separated list
        pub deploy_statuses: Vec<String>, // From GHA2DB_DEPLOY_STATUSES, webhook tool, default "Passed,Fixed", - comma separated list
        pub deploy_results: Vec<i16>, // From GHA2DB_DEPLOY_RESULTS, webhook tool, default "0", - comma separated list
        pub deploy_types: Vec<String>, // From GHA2DB_DEPLOY_TYPES, webhook tool, default "push", - comma separated list
        pub project_root: String, // From GHA2DB_PROJECT_ROOT, webhook tool, no default, must be specified to run webhook tool
        pub exec_fatal: bool, // default true, set this manually to false to avoid lib.ExecCommand calling os.Exit() on failure and return error instead
        pub exec_quiet: bool, // default false, set this manually to true to have quite exec failures (for example `get_repos` git-clones or git-pulls on errors).
        pub exec_output: bool, // default false, set to true to capture commands STDOUT
        pub project: String, // From GHA2DB_PROJECT, gha2db_sync default "", You should set it to something like "kubernetes", "prometheus" etc.
        pub tests_yaml: String, // From GHA2DB_TESTS_YAML ./dbtest.sh tool, set other tests.yaml file, default is "tests.yaml"
        pub repos_dir: String,  // From GHA2DB_REPOS_DIR get_repos tool, default "~/devstats_repos/"
        pub process_repos: bool, // From GHA2DB_PROCESS_REPOS get_repos tool, enable processing (cloning/pulling) all devstats repos, default false
        pub process_commits: bool, // From GHA2DB_PROCESS_COMMITS get_repos tool, enable update/create mapping table: commit - list of file that commit refers to, default false
        pub external_info: bool, // From GHA2DB_EXTERNAL_INFO get_repos tool, enable outputing data needed by external tools (cncf/gitdm), default false
        pub projects_commits: String, // From GHA2DB_PROJECTS_COMMITS get_repos tool, set list of projects for commits analysis instead of analysing all, default "" - means all
        pub propagate_only_var: bool, // From GHA2DB_PROPAGATE_ONLY_VAR, if set the it will check ONLY="a b c" env variable and propagate it into other project filter variables if they're not set, for example GHA2DB_PROJECTS_COMMITS
        pub projects_yaml: String, // From GHA2DB_PROJECTS_YAML, many tools - set main projects file, default "projects.yaml"
        pub company_acq_yaml: String, // From GHA2DB_COMPANY_ACQ_YAML, import_affs tool, set non-standard "companies.yaml" file
        pub projects_override: HashMap<String, bool>, // From GHA2DB_PROJECTS_OVERRIDE, get_repos and ./devstats tools - for example "-pro1,+pro2" means never sync pro1 and always sync pro2 (even if disabled in `projects.yaml`).
        pub affiliations_json: String, // From GHA2DB_AFFILIATIONS_JSON, import_affs tool - set main affiliations file, default "github_users.json"
        pub exclude_repos: HashMap<String, bool>, // From GHA2DB_EXCLUDE_REPOS, gha2db tool, default "" - comma separated list of repos to exclude, example: "theupdateframework/notary,theupdateframework/other"
        pub input_dbs: Vec<String>, // From GHA2DB_INPUT_DBS, merge_dbs tool - list of input databases to merge, order matters - first one will insert on a clean DB, next will do insert ignore (to avoid constraints failure due to common data)
        pub output_db: String, // From GHA2DB_OUTPUT_DB, merge_dbs tool - output database to merge into
        pub mm_offset: i8, // From GHA2DB_TMOFFSET, gha2db_sync tool - uses time offset to decide when to calculate various metrics, default offset is 0 which means UTC, good offset for USA is -6, and for Poland is 1 or 2
        pub default_hostname: String, // "devstats.cncf.io"
        pub recent_range: String, // From GHA2DB_RECENT_RANGE, ghapi2db tool, default '12 hours'. This is a recent period to check open issues/PR to fix their labels and milestones.
        pub recent_repos_range: String, // From GHA2DB_RECENT_REPOS_RANGE, ghapi2db tool, default '1 day'. This is a recent period to check modified repositories.
        pub min_ghapi_points: i16, // From GHA2DB_MIN_GHAPI_POINTS, ghapi2db tool, minimum GitHub API points, before waiting for reset.
        pub max_ghapi_wait_seconds: i16, // From GHA2DB_MAX_GHAPI_WAIT, ghapi2db tool, maximum wait time for GitHub API points reset (in seconds).
        pub max_ghapi_retry: i16, // From GHA2DB_MAX_GHAPI_RETRY, ghapi2db tool, maximum wait retries
        pub ghapi_error_is_fatal: bool, // From GHA2DB_GHAPI_ERROR_FATAL, ghapi2db tool, make any GH API error fatal, default false
        pub skip_ghapi: bool, // From GHA2DB_GHAPISKIP, ghapi2db tool, if set then tool is skipping GH API calls (all: events (artificial events to make sure we are in sync with GH) and commits (enriches obfuscated GHA commits data)
        pub skip_api_events: bool, // From GHA2DB_GHAPISKIPEVENTS, ghapi2db tool, if set then tool is skipping GH API events sync
        pub skip_api_commits: bool, // From GHA2DB_GHAPISKIPCOMMITS, ghapi2db tool, if set then tool is skipping GH API commits enrichment
        pub skip_api_licenses: bool, // From GHA2DB_GHAPISKIPLICENSES, ghapi2db tool, if set then tool is skipping GH API licenses enrichment
        pub force_api_licenses: bool, // From GHA2DB_GHAPIFORCELICENSES, ghapi2db tool, if set, recheck licenses on repos that already have licenses fetched
        pub skip_api_langs: bool, // From GHA2DB_GHAPISKIPLANGS, ghapi2db tool, if set then tool is skipping GH API repos programming languages enrichment
        pub force_api_langs: bool, // From GHA2DB_GHAPIFORCELANGS, ghapi2db tool, if set, recheck programming languages on repos that already have them fetched
        pub skip_set_repos: bool, // From GHA2DB_GETREPOSSKIP, get_repos tool, if set then tool does nothing
        pub csv_file: String, // From GHA2DB_CSVOUT, runq tool, if set, saves result in this file
        pub compute_all: bool, // From GHA2DB_COMPUTE_ALL, all tools, if set then no period decisions are taken based on time, but all possible periods are recalculated
        pub actors_filter: bool, // From GHA2DB_ACTORS_FILTER gha2db tool, if enabled then actor filterning will be added, default false
        pub actors_allow: Regex, // From GHA2DB_ACTORS_ALLOW, gha2db tool, process JSON if actor matches this regexp, default "" which means skip this check
        pub actors_forbid: Regex, // From GHA2DB_ACTORS_FORBID, gha2db tool, process JSON if actor doesn't match this regexp, default "" which means skip this check
        pub skip_metrics: HashMap<String, bool>, // From GHA2DB_SKIP_METRICS, gha2db_sync tool, default "" - comma separated list of metrics to skip, as given by "sql: name" in the "metrics.yaml" file. Those metrics will be skipped.
        pub only_metrics: HashMap<String, bool>, // From GHA2DB_ONLY_METRICS, gha2db_sync tool, default "" - comma separated list of metrics to process, as given by "sql: name" in the "metrics.yaml" file. Only those metrics will be calculated.
        pub allow_broken_json: bool, // From GHA2DB_ALLOW_BROKEN_JSON, gha2db tool, default false. If set then gha2db skips broken jsons and saves them as jsons/error_YYYY-MM-DD-h-n-m.json (n is the JSON number (1-m) of m JSONS array)
        pub jsons_dir: String,       // From GHA2DB_JSONS_DIR, website_data tool, default "./jsons/"
        pub website_data: bool, // From GHA2DB_WEBSITEDATA, devstats tool, run website_data just after sync is complete, default false.
        pub skip_update_events: bool, // From GHA2DB_SKIP_UPDATE_EVENTS, ghapi2db tool, drop and recreate artificial events if their state differs, default false
        pub compute_periods: HashMap<String, HashSet<bool>>, // From GHA2DB_FORCE_PERIODS, gha2db_sync tool, force recompute only given periods, "y10:t,m:f,...", default ""
        pub auto_fetch_commits: bool, // From GHA2DB_NO_AUTOFETCHCOMMITS, ghapi2db, disable fetching from last enriched commit data, it will fetch from 'RecentRange instead, AutoFetchCommits is enabled by default
        pub skip_tags: bool, // From GHA2DB_SKIP_TAGS, gha2db_sync tool, skip calling tags tool, default false
        pub skip_annotations: bool, // From GHA2DB_SKIP_ANNOTATIONS, gha2db_sync tool, skip calling annotations tool, default false
        pub skip_columns: bool, // From GHA2DB_SKIP_COLUMNS, gha2db_sync tool, skip calling columns tool, default false
        pub run_columns: bool, // From GHA2DB_RUN_COLUMNS, gha2db_sync tool, force calling columns tool, default false
        pub skip_vars: bool, // From GHA2DB_SKIP_VARS, gha2db_sync tool, skip calling vars tool, default false
        pub skip_rand: bool, // From GHA2DB_SKIP_RAND, gha2db_sync tool, skip randomizing metrics calculation, default false
        pub exclude_vars: HashMap<String, bool>, // From GHA2DB_EXCLUDE_VARS, vars tool, default "" - comma separated list of variable names to exclude, example: "hostname,projects_health_partial_html"
        pub only_vars: HashMap<String, bool>, // From GHA2DB_ONLY_VARS, vars tool, default "" - comma separated list of variable names to write (and skip all others): "hostname,projects_health_partial_html", not used if empty
        pub skip_shared_db: bool, // From GHA2DB_SKIP_SHAREDDB, annotations tool, default false, will skip writing to shared_db (from projects.yaml) if set
        pub skip_pid_file: bool, // From GHA2DB_SKIP_PIDFILE, devstats tool, skip creating, checking and removing PID file
        pub skip_company_acq: bool, // From GHA2DB_SKIP_COMPANY_ACQ, import_affs tool, skip processing company acquisitions from companies.yaml file
        pub check_provision_flag: bool, // From GHA2DB_CHECK_PROVISION_FLAG, devstats tool - check if there is a 'provision' metric saved in 'gha_computed' table - if not, abort
        pub check_running_flag: bool, // From GHA2DB_CHECK_RUNNING_FLAG, devstats tool - check if there is a 'devstats_running' metric saved in 'gha_computed' table - if yes, abort
        pub set_running_flag: bool, // From GHA2DB_SET_RUNNING_FLAG, devstats tool - set 'devstats_running' flag on 'gha_computed' table while devstats cronjob is running
        pub max_running_flag_age: Duration, // From GHA2DB_MAX_RUNNING_FLAG_AGE, how log "running_flag" can be present for next devstats sync to treat it as orphan, default "9h"
        pub check_imported_sha: bool, // From GHA2DB_CHECK_IMPORTED_SHA, import_affs tool - check if given JSON was already imported using 'gha_imported_shas' table
        pub only_check_imported_sha: bool, // From GHA2DB_ONLY_CHECK_IMPORTED_SHA, import_affs tool - check if given JSON was already imported using 'gha_imported_shas' table, do not attempt to import, only return status: 3=imported, 0=not imported
        pub enable_metrics_drop: bool, // From GHA2DB_ENABLE_METRICS_DROP, if enabled will process each metric's 'drop:' property if present - use when regenerating affiliations data or reinitializing entire TSDB data
        pub http_timeout: i16, // From GHA2DB_HTTP_TIMEOUT, gha2db - data.gharchive.org timeout value in minutes, default 2
        pub http_retry: i8, // From GHA2DB_HTTP_RETRY, gha2db - data.gharchive.org data fetch retries, default 4 (each retry takes 1*timeout*N), so in default config it will try timeouts: 1min, 2min, 3min, but if timeout is 3 and retry is 2, it will try 3min, 6min
        pub project_scale: f64, // From GHA2DB_PROJECT_SCALE, calc_metric tool, project scale (default 1), some metrics can use this to adapt their SQLs to bigger/smaller projects
        pub pid_file_root: String, // From GHA2DB_PID_FILE_ROOT, devstats tool, use '/tmp/PidFileRoot.pid' as PID file, default 'devstats' -> '/tmp/devstats.pid'
        pub shared_db: String, // Currently annotations tool read this from projects.yaml:shared_db and if set, outputs annotations data to the sharded DB in addition to the current DB
        pub project_main_repo: String, // Used by annotations tool to store project's main repo name
        pub test_mode: bool,   // True when running tests
        pub can_reconnect: bool, // True, unless connecting to a custom database, in this case there can be multiple threads sharing context and we don't want to write to a random database
        pub commits_files_stats_enabled: bool, // True, can be disabled by GHA2DB_SKIP_COMMITS_FILES, get_repos tool
        pub commits_loc_stats_enabled: bool, // True, can be disabled by GHA2DB_SKIP_COMMITS_LOC, get_repos tool
        pub recalc_reciprocal: i8, // From GHA2DB_RECALC_RECIPROCAL: 1/RecalcReciprocal of recalc metric at given datetime, even if it should be calculated at this datetime, default 24 (means 4.1(6)%, or about once/day)
        pub max_histograms: i8, // From GHA2DB_MAX_HIST: maximum histogram concurrency, default: 0 - means unlimited
        pub max_run_duration: HashMap<String, [i16; 2]>, // From GHA2DB_MAX_RUN_DURATION, how log given programs can run and exist status after timeout, for example "tags:1h:0,calc_metric:12h:1"
        pub rand_compute_at_this_date: bool, // Use rand to decide if a given date period must be calculated at this date or not.
        pub refresh_commit_roles: bool, // From GHA2DB_REFRESH_COMMIT_ROLES - will process all commiths in DB and for every single one of them it will generate gha_commits_roles entries.
        pub allow_rand_tags_cols_compute: bool, // If set, then tags and columns will only be computed at random 0-5 hour, otherwise always when hour<6.
    }
    impl Ctx {
        pub fn new() -> Self {
            Default::default()
        }
    }
    impl Default for Ctx {
        fn default() -> Self {
            Ctx { debug: 0 }
        }
    }
}
