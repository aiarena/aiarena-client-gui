import React, {Component} from "react";
import Select from 'react-select'
import Button from 'react-bootstrap/Button'
import { trackPromise } from 'react-promise-tracker';
// eslint-disable-next-line no-unused-vars
import * as bs from 'bootstrap/dist/css/bootstrap.css';

import axios from "axios";
import ResultsTable from "./ResultsTable";
import {usePromiseTracker} from "react-promise-tracker";
import { invoke } from '@tauri-apps/api/tauri'

function changeToDictionary(v) {
    return {value: v, label: v}
}


document.addEventListener('DOMContentLoaded', () => {
    // This will wait for the window to load, but you could
    // run this function on whatever trigger you want
    invoke('close_splashscreen')
})

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


const customStyles = {

    option: (provided, state) => ({
        ...provided,
        background: 'white',
        borderBottom: '1px dotted pink',
        color: state.isSelected ? 'green' : 'black',
        padding: 20,
    }),
    singleValue: (provided, state) => {
        const opacity = state.isDisabled ? 0.5 : 1;
        const transition = 'opacity 300ms';
        return {...provided, opacity, transition};
    }

}

class Home extends Component {
    constructor(props) {
        super(props);
        this.handleInputChange = this.handleInputChange.bind(this);
        this.onSubmitHandler = this.onSubmitHandler.bind(this);
        this.loadAIArenaBots = this.loadAIArenaBots.bind(this);
        this.addInputToState = this.addInputToState.bind(this);
        this.getNewResultsData = this.getNewResultsData.bind(this);

    }

    state = {
        bots: [],
        maps: [],
        iterations: 1,
        ai_arena_bots_loaded: false,
        Results: [],
        Bot1: [],
        Bot2: [],
        Map: [],
        Visualize: false,
        Realtime: false
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
    componentDidMount() {
        trackPromise(
        axios.get("http://127.0.0.1:8082/get_bots",)
            .then((data) => {

                let obj = {bots: []};
                data.data.Bots.forEach(value => {
                    obj.bots.push(changeToDictionary(value));
                });
                this.setState(obj);

            }).catch(console.log));
        trackPromise(
        axios.get("http://127.0.0.1:8082/get_maps").then((data) => {
            let obj = {maps: []};
            data.data.Maps.forEach(value => {
                obj.maps.push(changeToDictionary(value));
            });
            this.setState(obj);
        }));
        this.getNewResultsData();

    }
    clearResults() {
        trackPromise(axios.post("http://127.0.0.1:8082/clear_results").catch(reason => console.log(reason)));
    }
    loadAIArenaBots(){
        if (!this.state.ai_arena_bots_loaded) {
            trackPromise(
            axios.get("http://127.0.0.1:8082/get_arena_bots").then((data) =>{

                let obj = {'bots': this.state.bots};
                let results = data.data.results;
                for (let i =0; i < data.data.count; i++){

                    obj.bots.push(changeToDictionary(results[i].name + ' (AI-Arena)'));

                }
                this.setState(obj);

                let obj2 = {'ai_arena_bots_loaded': true};
                this.setState(obj2);
            }).catch(reason => {console.log(reason);}));
        }
    }
    getNewResultsData(){
        axios.get("http://127.0.0.1:8082/get_results").then((data)=>{
            let obj = data.data;
            this.setState(obj);
        });
    }
    addInputToState = name => value => {
        let obj = {};
        obj[name]= value.map(x => {return x.value})
        console.log(obj);
        this.setState(obj);
    }
    onSubmitHandler(event) {
        event.preventDefault();
        console.log(this.state.Bot1);
        const data = {
            Bot1: this.state.Bot1,
            Bot2: this.state.Bot2,
            Map: this.state.Map,
            Iterations: this.state.iterations,
            Visualize: this.state.Visualize,
            Realtime: this.state.Realtime,
        };
        console.log(data);
        trackPromise(
        axios({
            method: 'post',
            url: 'http://127.0.0.1:8082/run_games',
            data: data,
            headers: {
                'content-type': 'json'
            }
        }));
    }
    render() {
        return (
            <div className="middle-pad">
                <LoadingIndicator/>
                <main>
                    <h1>Home</h1>
                    <br/>
                    <label className="switch">
                        <Button hidden={this.state.ai_arena_bots_loaded} onClick={this.loadAIArenaBots} variant="outline-light">Load AI-Arena Bots (requires API Token in Settings)</Button>
                        <span className="slider round"/>
                    </label><br/>
                    <form style={{textAlign: 'left', width: '50%'}} id="my_form_id" onSubmit={this.onSubmitHandler}>
                        <h3 style={{textAlign: 'left'}}>Bot 1: </h3>
                        <Select name="Bot1" label="Bot 1" closeMenuOnSelect={false}  options={this.state.bots} isMulti styles={customStyles} onChange={this.addInputToState('Bot1')}/>
                        <br/>
                        <h3 style={{textAlign: 'left'}}>Bot 2: </h3>
                        <Select name="Bot2" label="Bot 2" closeMenuOnSelect={false}  options={this.state.bots} isMulti styles={customStyles} onChange={this.addInputToState('Bot2')} />
                        <br/>
                        <h3 style={{textAlign: 'left'}}>Map:</h3>
                        <Select id="Map" label="Map" closeMenuOnSelect={false}  options={this.state.maps} isMulti styles={customStyles} onChange={this.addInputToState('Map')}/>
                        <br/>
                        <h3 style={{textAlign: 'left'}}>Iterations: </h3>
                        <div style={{textAlign: 'left'}}>
                            <input type="number" min={1} step={1} value={this.state.iterations}
                                   name="iterations" onChange={this.handleInputChange}/>
                        </div>
                        <br/>
                        <div style={{textAlign: 'left'}}>
                            <label>Visualize: </label><br/>
                            <input id="visualize_id" type="checkbox" name="Visualize" checked={this.state.Visualize} onChange={this.handleInputChange}/>
                            <br/>
                            <label>Realtime: </label><br/>
                            <input id="realtime_id" type="checkbox" name="Realtime" checked={this.state.Realtime} onChange={this.handleInputChange}/>

                            <span/>
                            <br/><br/>

                            <Button type="submit" variant={"outline-light"} block>Play</Button>


                        </div>
                        <div id='subscribe'>
                        </div>
                        <br/><br/>
                    </form>
                    {/*<div id="watch_id">*/}
                    {/*    <h1>Live Feed</h1>*/}
                    {/*    <a href="/watch">Watch</a><br/><br/><br/>*/}
                    {/*</div>*/}
                    <div className='Results'>
                        <h2>Results</h2>
                        <Button id="clear_results" variant={"outline-light"} onClick={this.clearResults} >Clear Results</Button>
                        <Button id="refresh_results_id" variant={"outline-light"} onClick={this.getNewResultsData()} >Refresh</Button>
                        <br/><br/>
                        <ResultsTable data={(this.state.Results||[])}/>
                    </div>
                </main>
            </div>
        );
    }
}


export default Home;
export {LoadingIndicator};
