import React, {Component} from "react";
import Button from "react-bootstrap/Button";
import {invoke} from "@tauri-apps/api/tauri";

import qs from 'qs'
import axios from 'axios'
import {LoadingIndicator} from "./Home";
import {trackPromise} from "react-promise-tracker";

class Settings extends Component {
    constructor(props) {
        super(props);
        this.handleInputChange = this.handleInputChange.bind(this);
        this.openFolderDialog = this.openFolderDialog.bind(this);
        this.onSubmitHandler = this.onSubmitHandler.bind(this);
    }

    handleInputChange(event) {
        let obj = {};
        if (event.target.name === "allow_debug") {
            obj[event.target.name] = event.target.checked;
        } else {
            obj[event.target.name] = event.target.value;
        }

        if (event.target.name === "max_game_time") {
            obj["game_time"] = this.getGameTimeFromMaxGameTime(event.target.value);
        }
        this.setState(obj);
    }

    openFolderDialog(event) {
        trackPromise(
        invoke('open_file_dialog').then((path) => {
            if (path !== "") {
                let obj = {event: ""};
                obj[event] = path;
                this.setState(obj);
            }

        }).catch(reason => console.log(reason)));
    }


    getGameTimeFromMaxGameTime(max_game_time) {

        return [
            Math.floor(max_game_time / 22.4 / 60 / 60),
            Math.floor(max_game_time / 22.4 / 60 % 60),
            Math.floor(max_game_time / 22.4 % 60)
        ]
            .join(':')
            .replace(/\b(\d)\b/g, '0$1');
    }


    state = {
        bot_directory_location: "",
        sc2_directory_location: "",
        replay_directory_location: "",
        API_token: "",
        max_game_time: 0,
        allow_debug: false,
        game_time: "",
        tauri_enabled: false,
        local_file_directory: ""
    }

    componentDidMount() {
        trackPromise(axios.get("http://127.0.0.1:8082/get_settings",)
            .then((data) => {
                data.data['game_time'] = this.getGameTimeFromMaxGameTime(data.data.max_game_time);

                this.setState(data.data);
            }).catch(console.log));
        trackPromise(invoke('tauri_test').then(enabled => {
            let obj = {'tauri_enabled': enabled};
            this.setState(obj);
        }).catch(() => {
            let obj = {'tauri_enabled': false};
            this.setState(obj);
        }));
        trackPromise(invoke('get_project_directory').then(path => {
            let obj = {'local_file_directory': `${path}`};
            this.setState(obj);
        }).catch((e) => {
            console.log(e);
            let obj = {'local_file_directory': ""};
            this.setState(obj);
        }))


    }

    onSubmitHandler(event) {
        event.preventDefault();
        const data = qs.stringify({
            bot_directory_location: this.state.bot_directory_location,
            sc2_directory_location: this.state.sc2_directory_location,
            replay_directory_location: this.state.replay_directory_location,
            API_token: this.state.API_token,
            max_game_time: this.state.max_game_time,
            allow_debug: this.state.allow_debug,
        });
        trackPromise(
        axios({
            method: 'post',
            url: 'http://127.0.0.1:8082/handle_data',
            data: data,
            headers: {
                'content-type': 'application/x-www-form-urlencoded;charset=utf-8'
            }
        }));
    }
    openDirectory = name => event => {
        event.preventDefault();
        trackPromise(
        invoke("open_directory", {path: name}));

    }

    render() {
        return (
            <div className="middle-pad">
                <LoadingIndicator/>
                <div>
                    <div style={{TextAlign: 'right'}}>
                        <a href="/non-existing" onClick={this.openDirectory(this.state.local_file_directory)}>{'App Data Directory: ' + this.state.local_file_directory}</a>
                    </div>
                    <main>
                        <h1>Settings</h1><br/><br/>

                        <label>Bots Location</label><br/>
                        <form onSubmit={this.onSubmitHandler}>
                            {/*action="http://127.0.0.1:8082/handle_data"*/}
                            {/*method="post"*/}
                            {/*encType="application/x-www-form-urlencoded"*/}
                            <input type="text" id="bot_directory_id_field" style={{width: '60%'}}
                                   name="bot_directory_location" value={this.state.bot_directory_location}
                                   onInput={this.handleInputChange}/><Button disabled={!this.state.tauri_enabled}
                                                                             variant="outline-light"
                                                                             onClick={() => this.openFolderDialog('bot_directory_location')}>Select
                            Folder</Button><br/>
                            <label>StarCraft II Install Location</label><br/>
                            <input type="text" id="sc2_directory_id_field" style={{width: '60%'}}
                                   name="sc2_directory_location" value={this.state.sc2_directory_location}
                                   onInput={this.handleInputChange}/><Button disabled={!this.state.tauri_enabled}
                                                                             variant="outline-light"
                                                                             onClick={() => this.openFolderDialog('sc2_directory_location')}>Select
                            Folder</Button><br/>
                            <label>Replay Save Location</label><br/>
                            <input type="text" id="replay_directory_id_field" style={{width: '60%'}}
                                   name="replay_directory_location" value={this.state.replay_directory_location}
                                   onInput={this.handleInputChange}/><Button disabled={!this.state.tauri_enabled}
                                                                             variant="outline-light"
                                                                             onClick={() => this.openFolderDialog('replay_directory_location')}>Select
                            Folder</Button><br/>
                            <label>AI-Arena API Token</label><br/>
                            <input id="API_token_id" type="text" style={{width: '60%'}} name="API_token"
                                   value={this.state.API_token} onInput={this.handleInputChange}/> <a
                            href="https://aiarena.net/profile/token/?" rel="noopener noreferrer"
                            target="_blank">API Token Link</a><br/><br/>

                            <label>Max Game Time</label><br/><input type="range" style={{width: '40%'}}
                                                                    name="max_game_time"
                                                                    id="max_game_time_id"
                                                                    value={this.state.max_game_time}
                                                                    min="1" max="432000"
                                                                    onInput={this.handleInputChange}/>
                            <input readOnly={true} className="unselectable-input" name="game_time" id="game_time_id"
                                   value={this.state.game_time} unselectable="on"/>
                            <br/><br/>
                            <label>Allow Debug:</label><br/><label className="switch">
                            <input id="checkboxInputId" type="checkbox" name="allow_debug" checked={this.state.allow_debug} onChange={this.handleInputChange}/>
                            <span className="slider round"/>
                        </label><br/><br/>
                            <a href="http://127.0.0.1:8082/ac_log/aiarena-client.log">Download AC Log</a>
                            <br/>
                            <Button variant="outline-light" type="submit">Submit</Button>
                        </form>
                    </main>
                </div>
            </div>
        );
    }
}

export default Settings;