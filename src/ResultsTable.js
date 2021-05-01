import React from "react";

export default class ResultsTable extends React.Component {

    constructor(props) {
        super(props);
        this.getHeader = this.getHeader.bind(this);
        this.getRowsData = this.getRowsData.bind(this);
        this.getKeys = this.getKeys.bind(this);
    }

    getKeys() {
        let data = this.props.data;
        if (data.length >0){
            return Object.keys(data[0]);
        }else{
            return [];
        }
    }

    getHeader() {
        const keys = this.getKeys();
        return keys.map((key, index) => {
            return <th key={key}>{key.toUpperCase()}</th>
        })
    }

    getRowsData() {
        const items = this.props.data;
        const keys = this.getKeys();
        return items.map((row, index)=>{
            return <tr key={index}><RenderRow key={index} data={row} keys={keys}/></tr>
        })
    }

    render() {
        return (
            <div>
                <table>
                    <thead>
                    <tr>{this.getHeader()}</tr>
                    </thead>
                    <tbody>
                    {this.getRowsData()}
                    </tbody>
                </table>
            </div>

        );
    }
}
const RenderRow = (props) => {
    return props.keys.map((key, index)=>{
        return <td key={index}>{props.data[key]}</td>
    })
}