import React, { Component } from 'react';
import './App.css';
import {
    Route,
    HashRouter,
    Link
} from "react-router-dom";
import Navbar from 'react-bootstrap/Navbar';
import Nav from 'react-bootstrap/Nav';

import Settings from "./Settings";
import Home from "./Home";
import Debug from "./Debug";


class App extends Component{
  render() {
    return (
        <HashRouter>
        <div>
            <Navbar expand="md" bg="transparent" variant="dark">
                <Navbar.Brand>Match Runner</Navbar.Brand>
                <Nav className="mr-auto">
                    <Nav.Link as={Link} exact to="/">Home</Nav.Link>
                    <Nav.Link as={Link} to="/settings">Settings</Nav.Link>
                    <Nav.Link as={Link} to="/debug">Debug</Nav.Link>
                    {/*<Nav.Link as={Link} to="/contact">Pricing</Nav.Link>*/}
                </Nav>
            </Navbar>
            <br />
          {/*<ul className="header">*/}
          {/*  <li><NavLink exact to="/">Home</NavLink></li>*/}
          {/*  <li><NavLink to="/settings">Settings</NavLink></li>*/}
          {/*  <li><NavLink to="/contact">Contact</NavLink></li>*/}
          {/*</ul>*/}
          <div className="content">
              <Route exact path="/" component={Home}/>
              <Route path="/settings" component={Settings}/>
              <Route path="/debug" component={Debug}/>
          </div>
        </div>
        </HashRouter>
    );
  }
}


export default App;
