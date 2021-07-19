import { Navbar, Container } from "react-bootstrap";

import Config from "./components/Config";
import Targets from "./components/Targets";
import Mappings from "./components/Mappings";

function App() {
    return (
        <Container fluid>
            <Navbar bg="light" expand="lg">
                <Container>
                    <Navbar.Brand>taus</Navbar.Brand>
                    <Navbar.Text>
                        the <i>Tauri auto-update server</i>
                    </Navbar.Text>
                </Container>
            </Navbar>
            <Container>
                <Config />
                <Targets />
                <Mappings />
            </Container>
        </Container>
    );
}

export default App;
