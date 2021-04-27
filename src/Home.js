import React, {Component} from "react";
import Select from 'react-select'
import Button from 'react-bootstrap/Button'
// eslint-disable-next-line no-unused-vars
import * as bs from 'bootstrap/dist/css/bootstrap.css';

import axios from "axios";
function changeToDictionary(v) {
    return {value: v, label: v}
}

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
    constructor() {
        super();
        this.handleInputChange = this.handleInputChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
        this.loadAIArenaBots = this.loadAIArenaBots.bind(this);
    }

    state = {
        bots: [],
        maps: [],
        iterations_id: 1,
        ai_arena_bots_loaded: false
    }

    handleInputChange(event) {
        let obj = {};
        obj[event.target.name] = event.target.value;
        this.setState(obj);
    }

    componentDidMount() {
        axios.get("http://127.0.0.1:8082/get_bots",)
            .then((data) => {

                let obj = {bots: []};
                data.data.Bots.forEach(value => {
                    obj.bots.push(changeToDictionary(value));
                });
                this.setState(obj);

            }).catch(console.log);
        axios.get("http://127.0.0.1:8082/get_maps").then((data) => {
            let obj = {maps: []};
            data.data.Maps.forEach(value => {
                obj.maps.push(changeToDictionary(value));
            });
            this.setState(obj);
        });
    }
    loadAIArenaBots(){
        if (!this.state.ai_arena_bots_loaded) {
            axios.get("http://127.0.0.1:8082/get_arena_bots").then((data) =>{

                let obj = {'bots': this.state.bots};
                console.log(obj);
                let results = data.data.results;
                for (var i =0; i < data.data.count; i++){
                    console.log(i);
                    obj.bots.push(changeToDictionary(results[i].name + ' (AI-Arena)'));
                    console.log(results[i].name + ' (AI-Arena)');
                }
                this.setState(obj);
                console.log(obj);
                let obj2 = {'ai_arena_bots_loaded': true};
                this.setState(obj2);
            }).catch(reason => {console.log(reason);});
        }
    }
    handleSubmit(event) {
        console.log(event);
        event.preventDefault();
    }
    render() {

        return (
            <div className="middle-pad">

                <main>
                    <h1>Home</h1>
                    <br/>
                    <label className="switch">
                        <Button hidden={this.state.ai_arena_bots_loaded} onClick={this.loadAIArenaBots} variant="outline-light">Load AI-Arena Bots (requires API Token in Settings)</Button>
                        <span className="slider round"/>
                    </label><br/>
                    <form style={{textAlign: 'left', width: '50%'}} id="my_form_id" onSubmit={this.handleSubmit}>
                        <h3 style={{textAlign: 'left'}}>Bot 1: </h3>
                        <Select name="Bot1" label="Bot 1" options={this.state.bots} isMulti styles={customStyles}/>
                        <br/>
                        <h3 style={{textAlign: 'left'}}>Bot 2: </h3>
                        <Select name="Bot2" label="Bot 2" options={this.state.bots} isMulti styles={customStyles}/>
                        <br/>
                        <h3 style={{textAlign: 'left'}}>Map:</h3>
                        <Select id="Map" label="Map" options={this.state.maps} isMulti styles={customStyles}/>
                        <br/>
                        <h3 style={{textAlign: 'left'}}>Iterations: </h3>
                        <div style={{textAlign: 'left'}}>
                            <input type="number" min={1} step={1} value={this.state.iterations_id}
                                   name="iterations_id" onChange={this.handleInputChange}/>
                        </div>
                        <br/>
                        <div style={{textAlign: 'left'}}>
                            <label>Visualize: </label><br/>
                            <input id="visualize_id" type="checkbox" name="visualize"/>
                            <br/>
                            <label>Realtime: </label><br/>
                            <input id="realtime_id" type="checkbox" name="realtime"/>

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
                        <Button id="clear_results" variant={"outline-light"} >Clear Results</Button>
                        <Button id="refresh_results_id" variant={"outline-light"} >Refresh</Button>
                        <br/>
                        <body onLoad="generateDynamicTable()">
                        <div id="myResults">
                            <p/>
                        </div>
                        </body>
                    </div>
                </main>
            </div>
        );
    }
}

export default Home;