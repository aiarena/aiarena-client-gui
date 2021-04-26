import React, {Component} from "react";
import Button from "react-bootstrap/Button";
import {invoke} from "@tauri-apps/api/tauri";

const axios = require('axios');

class Settings extends Component {
    constructor(props) {
        super(props);
        this.handleInputChange = this.handleInputChange.bind(this);
        this.openFolderDialog = this.openFolderDialog.bind(this);
    }

    handleInputChange(event) {
        let obj = {};
        obj[event.target.name] = event.target.value;

        if (event.target.name === "max_game_time") {
            obj["game_time"] = this.getGameTimeFromMaxGameTime(event.target.value);
        }
        this.setState(obj);
    }
    openFolderDialog(event){
        invoke('my_custom_command').then((path) => {
            if (path !== ""){
                let obj = {event:""};
                obj[event] = path;
                this.setState(obj);
            }

        }).catch(reason => console.log(reason));
    }
    testTauri(){

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
        api_token: "",
        max_game_time: 0,
        allow_debug: "",
        game_time: "",
        tauri_enabled: false
    }

    componentDidMount() {
        axios.get("http://127.0.0.1:8082/get_settings",)
            .then((data) => {
                data.data['game_time'] = this.getGameTimeFromMaxGameTime(data.data.max_game_time);
                this.setState(data.data);
            }).catch(console.log);
        invoke('tauri_test').then(enabled =>{
            let obj = {'tauri_enabled': enabled};
            this.setState(obj);
        }).catch(() => {
            let obj = {'tauri_enabled': false};
            this.setState(obj);
        })


    }

    render() {
        return (
            <div className="middle-pad">
                <div >
                    <main >
                        <h1>Settings</h1><br/><br/>
                        <label>Bots Location</label><br/>
                        <form action="http://127.0.0.1:8082/handle_data"
                              method="post"
                              encType="application/x-www-form-urlencoded" >
                            <input type="text" id="bot_directory_id_field" style={{width: '60%'}}
                                   name="bot_directory_location" value={this.state.bot_directory_location}
                                   onInput={this.handleInputChange}/><Button disabled={!this.state.tauri_enabled} variant="outline-light" onClick={() => this.openFolderDialog('bot_directory_location')}>Select Folder</Button><br/>
                            <label>StarCraft II Install Location</label><br/>
                            <input type="text" id="sc2_directory_id_field" style={{width: '60%'}}
                                   name="sc2_directory_location" value={this.state.sc2_directory_location}
                                   onInput={this.handleInputChange}/><Button disabled={!this.state.tauri_enabled} variant="outline-light" onClick={() => this.openFolderDialog('sc2_directory_location')}>Select Folder</Button><br/>
                            <label>Replay Save Location</label><br/>
                            <input type="text" id="replay_directory_id_field" style={{width: '60%'}}
                                   name="replay_directory_location" value={this.state.replay_directory_location}
                                   onInput={this.handleInputChange}/><Button disabled={!this.state.tauri_enabled} variant="outline-light" onClick={() => this.openFolderDialog('replay_directory_location')}>Select Folder</Button><br/>
                            <label>AI-Arena API Token</label><br/>
                            <input id="API_token_id" type="text" style={{width: '60%'}} name="API_token"
                                   value={this.state.api_token} onInput={this.handleInputChange}/> <a
                            href="https://aiarena.net/profile/token/?" rel="noopener noreferrer"
                            target="_blank">API Token Link</a><br/><br/>

                            <label>Max Game Time</label><br/><input  type="range" style={{width: '40%'}}
                                                                    name="max_game_time"
                                                                    id="max_game_time_id"
                                                                    value={this.state.max_game_time}
                                                                    min="1" max="432000"
                                                                    onInput={this.handleInputChange}/>
                            <input readOnly='true' className="unselectable-input" name="game_time" id="game_time_id" value={this.state.game_time} unselectable="on"/>
                            <br/><br/>
                            <label>Allow Debug:</label><br/><label className="switch">
                            <input id="checkboxInputId" type="checkbox" name="allow_debug"
                                   value={this.state.allow_debug} onInput={this.handleInputChange}/>
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