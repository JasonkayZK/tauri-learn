'use client'

import React from "react";
import {invoke} from "@tauri-apps/api";

export default class Greet extends React.Component {

    props: {
        name: string;
    };

    state: {
        greeting: string
    }

    constructor(props: {name: string}) {
        super(props);

        this.props = {
            name: props.name
        }

        this.state = {
            greeting: ''
        };
    }

    async componentDidMount() {
        const greeting = await invoke<string>('greet', {name: this.props.name});
        this.setState({greeting: greeting});
    }

    render() {
        return (
            <h1>
                {this.state.greeting}
            </h1>
        )
    }
}
