pub mod server;
pub mod types;
pub mod utils;
pub use server::server::FrameWork;

#[cfg(test)]
mod tests {
    use crate::server::server::{HandlerResult, FrameWork};
    use crate::server::unstarted_server::UnstartedServer;
    use crate::types::http_types::http_request_type::HttpRequest;
    use crate::types::http_types::http_response_type::HttpResponse;

    use std::env;
    use std::process::Command;

    static OVERRIDE_DEBUG_FILE_PATH: &'static str = "../override_debug_file.txt";
    static STATIC_HOST_PATH: &'static str = "/test_files/";
    static TESTING_HOST: &'static str = "127.0.0.1";
    static TESTING_PORT: &'static str = "7178";
    static OUTWARD_REPSONSE: &'static str = "outward fucntion works";

    //NOTE: test should be ran in groups to avoide confliction that will cause test to give false
    //failes, (caused by trying to start multiple instances of servers)
    //TEST GROUPS:
    //add - will test that user defined data is being added to the unstartedServer struct correctly
    //override - will test that the used defined config data is being applied to config struct
    //apply - will test if the user added functionality is being applied correctly and apporitate
    //        response types are returned
    //NOTE: core group need to be ran seperatly be avoin `stack overflow` due to confliction
    //core - will test basic functionality with default settings
    //outward - will test the fucnionality of the created endpoints (like querying the endpoints
    //          created by the server)

    /// just creates the basic server for testing purposes
    fn init_server() -> &'static mut UnstartedServer {
        let mut config = FrameWork::new_config();
        config.set_host(TESTING_HOST).set_port(7178);
        config.use_default_404_handler();
        config.enable_debug_mode();
        let http = FrameWork::new(config);
        return http;
    }
    #[test]
    fn add_static_host_dir() {
        let http = self::init_server();
        let host_result = http.static_host("/test_files", true);
        assert_eq!(host_result, Ok(()));
        return;
    }
    #[test]
    fn add_hello_route() {
        let http = self::init_server();
        let hello_route_result = http.add_handler(
            "/hello",
            "GET",
            |_req: &mut HttpRequest, res: &mut HttpResponse| -> HandlerResult {
                let _val = res
                    .set_status_code(200)?
                    .text_to_body(_req.body.to_string())?;
                Ok(())
            },
        );
        assert_eq!(hello_route_result, Ok(()));
        return;
    }

    #[test]
    fn add_invalid_method() {
        let http = self::init_server();
        let fail = http
            .add_handler(
                "/fail",
                "NEXT",
                |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
            )
            .unwrap_err();
        assert_eq!(fail.identify(), "AppenedDataError".to_string());
        let fail = http
            .add_handler(
                "/fail",
                "INVALID",
                |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
            )
            .unwrap_err();
        assert_eq!(fail.identify(), "ConstructError".to_string());
    }

    #[test]
    fn add_to_many_handlers() {
        // default limit is 4
        let http = self::init_server();
        let _ = http.add_handler(
            "/tomany",
            "GET",
            |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
        );
        let _ = http.add_handler(
            "/tomany",
            "PUT",
            |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
        );
        let _ = http.add_handler(
            "/tomany",
            "POST",
            |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
        );
        let _ = http.add_handler(
            "/tomany",
            "DELETE",
            |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
        );
        let fail = http
            .add_handler(
                "/tomany",
                "GET",
                |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
            )
            .unwrap_err();
        assert_eq!(fail.identify(), "AppenedDataError".to_string());
        return;
    }
    #[test]
    fn add_to_many_middlewares() {

        // default limit is 5
        let http = self::init_server();
        let _ = http.add_middleware_handler(
            "/tomany",
            |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
        );
        let _ = http.add_middleware_handler(
            "/tomany",
            |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
        );
        let _ = http.add_middleware_handler(
            "/tomany",
            |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
        );
        let _ = http.add_middleware_handler(
            "/tomany",
            |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
        );
        let _ = http.add_middleware_handler(
            "/tomany",
            |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
        );
        let fail = http
            .add_middleware_handler(
                "/tomany",
                |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
            )
            .unwrap_err();
        println!("{:?}", fail);
        assert_eq!(fail.identify(), "AppenedDataError".to_string());
        return;
    }

    #[test]
    fn override_middleware_limit() {
        
        // default limit is 5
        let mut config = FrameWork::new_config();
        config
            .set_host("localhost")
            .set_port(7171)
            .set_middleware_limit(1);
        let http = FrameWork::new(config);
        let _ = http.add_middleware_handler(
            "/tomany",
            |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
        );
        let fail = http
            .add_middleware_handler(
                "/tomany",
                |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
            )
            .unwrap_err();
        assert_eq!(fail.identify(), "AppenedDataError".to_string());
        return;
    }

    #[test]
    fn override_debug_file_path() {
        // default limit is 5
        let mut config = FrameWork::new_config();
        config
            .set_host("localhost")
            .set_port(7171)
            .set_deubug_file_path(OVERRIDE_DEBUG_FILE_PATH);
        let _ = FrameWork::new(config);
        assert_eq!(
            std::path::Path::new(OVERRIDE_DEBUG_FILE_PATH).is_file(),
            true
        );
        std::fs::remove_file(OVERRIDE_DEBUG_FILE_PATH).expect(
            format!(
                "failed to remove the override debug file, located at {}",
                OVERRIDE_DEBUG_FILE_PATH
            )
            .as_str(),
        );
        return;
    }


    #[test]
    fn add_redundent_route() {
        let http = self::init_server();
        let _ = http.add_handler(
            "/pred",
            "GET",
            |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
        );
        let result = http
            .add_handler(
                "/pred",
                "GET",
                |_req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult { Ok(()) },
            )
            .unwrap_err();
        assert_eq!(result.identify(), "RedundentDataError".to_string());
        return;
    }

    #[test]
    fn add_middleware_test() {
        // default limit is 5
        let http = self::init_server();
        let _ = http.add_middleware_handler(
            "/hello",
            |req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult {
                println!("this is the middleware 2");
                req.body = "THIS THE MIDDLEWARE MODIFYING THE REQUEST 2".to_string();
                Ok(())
            },
        );
        return;
    }

    #[test]
    fn apply_middleware() {
        // will add a middleware and make sure the reponse struct returned is correct
        let http = self::init_server();
        let _ = http.add_middleware_handler(
            "/hello",
            |req: &mut HttpRequest, _res: &mut HttpResponse| -> HandlerResult {
                println!("this is the middleware 2");
                req.body = "THIS THE MIDDLEWARE MODIFYING THE REQUEST 2".to_string();
                Ok(())
            },
        );
    }

    #[test]
    fn apply_handler() {
        // will add a handler and make sure the reponse struct returned is correct
        let http = self::init_server();
        let _ = http.add_handler(
            "/hello",
            "GET",
            |_req: &mut HttpRequest, res: &mut HttpResponse| -> HandlerResult {
                let _val = res
                    .set_status_code(200)?
                    .text_to_body(_req.body.to_string())?;
                Ok(())
            },
        );
    }

    #[test]
    fn apply_staic_file() {
        // will add file to static dir and make sure it is returned in a response
        let http = self::init_server();
        let r = http.static_host(STATIC_HOST_PATH, true);
        assert!(r.is_ok(), "error type: {},", r.unwrap_err().identify());
    }

    #[test]
    fn apply_custom_config() {
        let mut config = FrameWork::new_config();
        config.set_host("localhost").set_port(7178);

        assert_eq!(config.expose_host(), TESTING_HOST);
        assert_eq!(config.expose_port(), TESTING_PORT);
        return;
    }

    #[test]
    /// test that the server will start with default settings
    fn core_start() {
        let mut config = FrameWork::new_config();
        config.set_port(7171).set_host("localhost");
        let http = FrameWork::new(config);
        let successful_start = match http.start() {
            Ok(_) => true,
            Err(_) => false,
        };
        assert!(successful_start, "server failed to start");
        return;
    }

    #[test]
    /// need to shut down the server
    fn core_end() {
        let mut config = FrameWork::new_config();
        config.set_port(6969).set_host("localhost");
        let http = FrameWork::new(config);
        let server = http.start().unwrap();
        let stopped = server.kill_server();
        assert_eq!(stopped.final_error.identify(), "ServerInfo");
        return;
    }

    fn outward_server_config() -> &'static mut UnstartedServer {
        let mut config = FrameWork::new_config();
        config.set_host(TESTING_HOST).set_port(7178);
        config.enable_debug_mode();
        let http = FrameWork::new(config);
        return http;
    }

    #[test]
    // test the functionallity of the routes created
    fn outward_functionallity() {
        let http = outward_server_config();

        let static_error = http.static_host(STATIC_HOST_PATH, true);

        assert_eq!(static_error.unwrap(), (), "failed to host the static dir");

        let route_error = http.add_handler(
            "/outward",
            "GET",
            |_req: &mut HttpRequest, res: &mut HttpResponse| -> HandlerResult {
                res.set_status_code(200)?
                    .text_to_body(OUTWARD_REPSONSE.to_owned())?;
                return Ok(());
            },
        );
        assert_eq!(
            route_error.unwrap(),
            (),
            "failed to create the http handler"
        );

        match http.start() {
            Ok(s) => {
                // test http route
                let http_command = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(["/C", "echo dont know windown curl program"])
                        .output()
                        .expect("failed to execute process")
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg(format!("curl {}:{}/outward", TESTING_HOST, TESTING_PORT))
                        .output()
                        .expect("failed to execute process")
                };
                let server_response =
                    String::from_utf8(http_command.stdout).expect("failed to decode response");
                assert_eq!(
                    server_response,
                    OUTWARD_REPSONSE.to_owned(),
                    "endpoint hit but wrong response returned"
                );

                //test static file route
                let static_file_command = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(["/C", "echo dont know windown curl program"])
                        .output()
                        .expect("failed to execute process")
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg(format!("curl {}:{}/nervous", TESTING_HOST, TESTING_PORT))
                        .output()
                        .expect("failed to execute process")
                };
                let path = env::current_dir().unwrap();
                let local_path = format!(
                    "{}{}{}",
                    path.to_str().unwrap(),
                    STATIC_HOST_PATH,
                    "nervous.jpg"
                );
                let file =
                    std::fs::read(local_path).expect("failed to read the file from local machine");
                assert_eq!(
                    static_file_command.stdout, file,
                    "wrong file returned from server"
                );

                s.kill_server();
            }
            Err(_) => {
                panic!("server failed to start in outward test")
            }
        };
    }
}
