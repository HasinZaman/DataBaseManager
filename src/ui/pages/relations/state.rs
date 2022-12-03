use state_machine_future::*;

use regex::{Regex};
use lazy_static::lazy_static;

use crate::backend::{sql::{SQL}, data_base::DataBase};

#[derive(StateMachineFuture)]
enum UpdateSchemaState {
    #[state_machine_future(start, transitions(ExecuteFile, ExecuteCMD))]
    GetInput{input: String},

    #[state_machine_future(transitions(Success))]
    ExecuteFile{cmds: Vec<SQL>},

    #[state_machine_future(transitions(Success))]
    ExecuteCMD{cmd: SQL},

    #[state_machine_future(ready)]
    Success(()),

    #[state_machine_future(error)]
    Fail(bool),
}

impl PollUpdateSchemaState for UpdateSchemaState{
    fn poll_get_input<'smf_poll_state,'smf_poll_context>(input: &'smf_poll_state mut __smf_update_schema_state_state_machine_future::RentToOwn<'smf_poll_state,GetInput>) -> __smf_update_schema_state_futures::Poll<AfterGetInput,bool>  {
        
        let get_input = input.take();

        let input = get_input.input;


        lazy_static! {
            static ref FILE_INPUT_REGEX: Regex = Regex::new("^[Ff][Ii][Ll][Ee]:[a-zA-Z][a-zA-Z0-9:/\\._]*.[Ss][Qq][Ll]$").unwrap();
        };

        let sql_cmd = SQL::from(&input);


        if let Ok(sql_cmd) = sql_cmd {
            //sql cmd
            let from_cmd = ExecuteCMD {
                cmd: sql_cmd,
            };

            transition!(from_cmd);
        }
        else if FILE_INPUT_REGEX.is_match(&input) {
            //file input
            match SQL::from_file(&input) {
                Ok(cmds) => {
                    let from_file = ExecuteFile {
                        cmds: cmds,
                    };
        
                    transition!(from_file);
                },
                Err(_) => {
                    return Err(false);
                },
            }   
        }
        else {
            //let err = Fail(false);

            return Err(false)
        }
    }

    fn poll_execute_file<'smf_poll_state,'smf_poll_context>(cmds: &'smf_poll_state mut __smf_update_schema_state_state_machine_future::RentToOwn<'smf_poll_state,ExecuteFile>) -> __smf_update_schema_state_futures::Poll<AfterExecuteFile,bool>  {
        let cmds = cmds.take().cmds;

        let result = DataBase::from_env()
            .unwrap()
            .execute_multiple(&cmds);

        if let Err(_) = result {
            return Err(false);
        }

        let finished = Success(());

        transition!(finished);
    }

    fn poll_execute_cmd<'smf_poll_state,'smf_poll_context>(cmd: &'smf_poll_state mut __smf_update_schema_state_state_machine_future::RentToOwn<'smf_poll_state,ExecuteCMD>) -> __smf_update_schema_state_futures::Poll<AfterExecuteCMD,bool>  {
        let cmd = cmd.take().cmd;

        let result = DataBase::from_env()
            .unwrap()
            .execute(&cmd, |_| ());


        if let Err(err) = result {
            return Err(false);
        }

        let finished = Success(());

        transition!(finished);
    }
}