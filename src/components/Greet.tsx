'use client'

import React, {Component} from "react";
import {invoke} from "@tauri-apps/api";

interface GreetProps {
    name: string;
}

interface GreetState {
    greeting: string;
}

export default class Greet extends Component<GreetProps, GreetState> {

    constructor(props: GreetProps) {
        super(props);

        this.state = {
            greeting: ''
        };
    }

    componentDidMount() {
        this.calculateGreet().then((resp) => this.setState({ greeting: resp}));
    }

    async calculateGreet(): Promise<string> {
        return invoke<string>('greet', {name: this.props.name})
            .then(resp => {
                return resp
            })
            .catch(err => {
                return err
            });
    }

    render() {
        const { greeting } = this.state;

        return (
            <div>
                <h1>{greeting}</h1>
            </div>
        );
    }
}
