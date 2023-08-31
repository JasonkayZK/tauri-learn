import React from 'react';
import '@/css/App.css';
import Greet from "@/components/Greet";
import ErrorHandle from "@/components/ErrorHandle";
import AsyncCommand from "@/components/AsyncCommand";

import "@/event/GlobalEvent";

function App() {
    return (
        <div className="App">
            <Greet name='Next.js'/>

            <ErrorHandle></ErrorHandle>

            <AsyncCommand bound={100}></AsyncCommand>
        </div>
    );
}

export default App;
