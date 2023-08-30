'use client'

import React from "react";
import {invoke} from "@tauri-apps/api";

export default class ErrorHandle extends React.Component {

    state: {
        msg: string
    }

    constructor(props: any) {
        super(props);

        this.state = {
            msg: ''
        };
    }

    async componentDidMount() {

        let msg = await invoke('error_handle_command')
            .then((message) => {
                return message;
            })
            .catch((error) => {
                return error;
            })

        this.setState({msg: msg})
    }

    render() {
        return (
            <h1>
                {this.state.msg}
            </h1>
        )
    }
}
