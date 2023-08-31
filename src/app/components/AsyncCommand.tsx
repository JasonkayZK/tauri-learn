'use client'

import React from "react";
import {invoke} from "@tauri-apps/api";

interface AsyncCommandProps {
    bound: number
}

interface AsyncCommandState {
    msg: string
}

export default class AsyncCommand extends React.Component<AsyncCommandProps, AsyncCommandState> {

    constructor(props: any) {
        super(props);

        this.state = {
            msg: ''
        };
    }

    async componentDidMount() {
        let msg = await invoke('async_command',
            {value: 'Solution of 1+..+' + this.props.bound, bound: this.props.bound})
            .then((res) => {
                return res as string;
            })
            .catch((err) => {
                return err;
            })

        await invoke('get_window_command');

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
