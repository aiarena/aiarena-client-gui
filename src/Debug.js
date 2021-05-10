import React, {Component} from "react";
import Select from 'react-select'
import Button from 'react-bootstrap/Button'
import { trackPromise } from 'react-promise-tracker';
// eslint-disable-next-line no-unused-vars
import * as bs from 'bootstrap/dist/css/bootstrap.css';
import {usePromiseTracker} from "react-promise-tracker";
import {invoke} from "@tauri-apps/api/tauri";


const LoadingIndicator = props => {
    const { promiseInProgress } = usePromiseTracker();
    return (
         promiseInProgress &&
        <div className="overlay">
            <div className="overlay__wrapper">
                <img className="overlay__spinner" src="ai-arena_logo_spinning.gif" alt=""/>
            </div>
        </div>
            );
};



class Debug extends Component {
    constructor(props) {
        super(props);
        this.handleInputChange = this.handleInputChange.bind(this);
        this.onSubmitHandler = this.onSubmitHandler.bind(this);
        this.addInputToState = this.addInputToState.bind(this);
        this.restartApp = this.restartApp.bind(this);
        this.openDirectory = this.openDirectory.bind(this);

    }

    state = {
        debug_logs_directory : "Does not exist",
       libraries: {
           mio: "off",
           actix_server: "off",
           aiarena_client_gui_backend_lib: "off",
           rust_ac: "off",
           frontend: "info",
           all: "off"
       },
        options:[
            {
                label: "off", value:"off"
            },
            {
                label: "info", value: "info"
            },
            {
                label: "debug", value: "debug"
            },
            {
                label: "trace", value: "trace"
            },
            {
                label: "error", value: "error"
            },
        ]
    }

    handleInputChange(event) {
        let obj = {};
        if (event.target.name === "allow_debug") {
            obj[event.target.name] = event.target.checked;
        } else {
            obj[event.target.name] = event.target.value;
        }
        this.setState(obj);
    }
    restartApp(event){
        let env_args = "";
        for (const key in this.state.libraries){
            if (key ==="frontend"){
                if (env_args.length > 0){
                    env_args = env_args.concat(",");
                }
                env_args = env_args.concat(this.state.libraries[key]);
            }else{
                if (env_args.length > 0){
                    env_args = env_args.concat(",");
                }
                env_args = env_args.concat(key,"=",this.state.libraries[key]);
            }
        }
        trackPromise(invoke("restart_app_with_logs", {envVar: env_args}));

    }
    componentDidMount() {
        trackPromise(invoke('get_debug_logs_directory').then(path => {
            let obj = {'debug_logs_directory': `${path}`};
            this.setState(obj);
        }).catch((e) => {
            console.log(e);
            let obj = {'debug_logs_directory': ""};
            this.setState(obj);
        }))
    }
    addInputToState = name => value => {
        let obj = this.state.libraries;
        console.log(name);
        console.log(value);
        if (name === "all"){
            obj = this.state.libraries;
            for (const key in obj){
                if (key !== "mio" && key !=="actix_server") {
                    obj[key] = value.value;
                }
            }
        }else{
            obj[name] = value.value;
        }
        console.log(obj);
        this.setState(obj);

    }
    onSubmitHandler(event) {
        event.preventDefault();
    }
    openDirectory = name => event => {
        event.preventDefault();
        trackPromise(
            invoke("open_directory", {path: name}));

    }
    render() {
        const items = []

        for (const value in this.state.libraries) {
            items.push(<label key={value}>{value}</label>);
            items.push(<Select options={this.state.options} placeholder={this.state.libraries[value]} value={this.state.libraries[value]} onChange={this.addInputToState(value)}/>);
        }
        return (
            <div className="middle-pad">
                <LoadingIndicator/>
                <main>
                    <h1>Debug</h1>
                    <div style={{TextAlign: 'right'}}>
                        <a href="/non-existing" onClick={this.openDirectory(this.state.debug_logs_directory)}>{'Debug Logs Directory: ' + this.state.debug_logs_directory}</a>
                    </div>
                    <br/>
                    <p className="warning">Note: It is not advised to set mio's logging to trace, since this shows the logs for selectors and context switching </p>
                    <br/>
                    <div style={{textAlign: 'left', width: '50%'}}>
                    {items}
                    </div>
                    <br/>
                    <Button onClick={this.restartApp} variant={"outline-light"}>Apply & Restart</Button>
                </main>
            </div>
        );
    }
}


export default Debug;

