use std::fs::File;

use state_machine_future::*;

use crate::backend::sql::QDL;

#[derive(StateMachineFuture)]
enum UpdateSchemaState {
    #[state_machine_future(start, transitions(FromFile, FromCMDLine))]
    GetInput{input: String},

    #[state_machine_future(transitions(ViewAffectedTuples, ConfirmQuery))]
    FromFile{file: File},

    #[state_machine_future(transitions(ViewAffectedTuples, ConfirmQuery))]
    FromCMDLine{cmd: String},

    #[state_machine_future(transitions(ConfirmQuery))]
    ViewAffectedTuples(QDL),

    #[state_machine_future(transitions(Ready))]
    ConfirmQuery(bool),

    #[state_machine_future(ready)]
    Ready(bool),

    #[state_machine_future(error)]
    Error(bool),
}

impl PollUpdateSchemaState for UpdateSchemaState{
    fn poll_get_input<'smf_poll_state,'smf_poll_context>(input: &'smf_poll_state mut __smf_update_schema_state_state_machine_future::RentToOwn<'smf_poll_state,GetInput>) -> __smf_update_schema_state_futures::Poll<AfterGetInput,bool>  {
        
        let get_input = input.take();

        let cmd = get_input.input;


        
        //try and compile to Query
        // match Query::from(&cmd){
        //     Ok(query) => {
        //         let from_cmd = FromCMDLine {
        //             cmd: ,
        //         };
        //     },
        //     Err(_) => todo!(),
        // }
        //check if input is file
            //send to cmd_line
        //check if input is command
            //send to file
        //else
            //send error
        todo!()
    }

    fn poll_from_file<'smf_poll_state,'smf_poll_context>(_: &'smf_poll_state mut __smf_update_schema_state_state_machine_future::RentToOwn<'smf_poll_state,FromFile>) -> __smf_update_schema_state_futures::Poll<AfterFromFile,bool>  {
        // convert queries strings into queries
            // if fail for one querry go to error
        // check all queries to see affected tuples
            // if greater than 0 then go show affected tuples
            // else go to confirm
        todo!()
    }

    fn poll_from_cmd_line<'smf_poll_state,'smf_poll_context>(_: &'smf_poll_state mut __smf_update_schema_state_state_machine_future::RentToOwn<'smf_poll_state,FromCMDLine>) -> __smf_update_schema_state_futures::Poll<AfterFromCMDLine,bool>  {
        // Convert string query
        // check all queries to see affected tuples
            // if greater than 0 then go show affected tuples
            // else go to confirm
        todo!()
    }

    fn poll_view_affected_tuples<'smf_poll_state,'smf_poll_context>(_: &'smf_poll_state mut __smf_update_schema_state_state_machine_future::RentToOwn<'smf_poll_state,ViewAffectedTuples>) -> __smf_update_schema_state_futures::Poll<AfterViewAffectedTuples,bool>  {
        // show affected queries
        // go to confirm
        todo!()
    }

    fn poll_confirm_query<'smf_poll_state,'smf_poll_context>(_: &'smf_poll_state mut __smf_update_schema_state_state_machine_future::RentToOwn<'smf_poll_state,ConfirmQuery>) -> __smf_update_schema_state_futures::Poll<AfterConfirmQuery,bool>  {
        todo!()
        // confirm or fail
    }
}