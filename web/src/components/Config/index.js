import { useState, useEffect } from "react";
import * as BS from "react-bootstrap";
import { useRefresh } from "../Utils";
import * as Formik from "formik";

import { validateRegex } from "../Utils";
import { updateConfig, getConfig } from "../../crud";

export default function Config() {
    const [error, setError] = useState();
    const [config, setConfig] = useState();
    const [ref, refresh] = useRefresh();
    useEffect(() => {
        getConfig()
            .then((data) => {
                Object.keys(data).forEach((k) => {
                    data[k] = data[k] === null ? "" : data[k];
                });
                setConfig(data);
            })
            .catch((err) => setError(err));
    }, [ref]);
    const validate = (values) => {
        const errors = {};
        let e = validateRegex(values.asset_regex);
        if (e) {
            errors.asset_regex = e;
        }
        return errors;
    };
    return (
        <>
            <h3> Config </h3>
            {!error && !config && (
                <BS.Spinner animation="border" role="status">
                    <span className="visually-hidden">Loading...</span>
                </BS.Spinner>
            )}
            {error && (
                <BS.Alert variant="danger">
                    Error connecting to backend: {`${error}`}
                </BS.Alert>
            )}
            {config && (
                <Formik.Formik
                    initialValues={config}
                    validate={validate}
                    onSubmit={(values, { touched, setSubmitting }) => {
                        ["github_repo", "github_user"].forEach((k) => {
                            if (values[k] === "") {
                                values[k] = null;
                            }
                        });
                        setSubmitting(true);
                        updateConfig(values)
                            .then(() => refresh())
                            .catch((error) => {
                                setError(error);
                            })
                            .finally(() => setSubmitting(false));
                    }}
                >
                    {({ errors }) => (
                        <Formik.Form as={BS.Form}>
                            <BS.Form.Group className="mb-3">
                                <BS.Form.Label>Fill Version</BS.Form.Label>
                                <Formik.Field
                                    name="fill_version"
                                    as={BS.Form.Control}
                                    type="text"
                                />
                                <BS.Form.Text className="text-muted">
                                    This is the version that newly discovered
                                    targets will be automatically mapped to
                                </BS.Form.Text>
                            </BS.Form.Group>

                            <BS.Form.Group className="mb-3">
                                <BS.Form.Label>Asset Regex</BS.Form.Label>
                                <Formik.Field
                                    isInvalid={errors.asset_regex}
                                    name="asset_regex"
                                    as={BS.Form.Control}
                                    type="text"
                                />
                                <BS.Form.Text className="text-muted">
                                    This first capture group of this regex
                                    determines the target an asset belongs to
                                </BS.Form.Text>
                                <BS.Form.Control.Feedback type="invalid">
                                    Invalid regex: {`${errors.asset_regex}`}
                                </BS.Form.Control.Feedback>
                            </BS.Form.Group>
                            <BS.Form.Group className="mb-3">
                                <BS.Form.Label>Github User</BS.Form.Label>
                                <Formik.Field
                                    isInvalid={errors.github_user}
                                    name="github_user"
                                    as={BS.Form.Control}
                                    type="text"
                                />
                                <BS.Form.Text className="text-muted">
                                    Name of the user or org that owns the repo
                                    (TODO: validate)
                                </BS.Form.Text>
                                <BS.Form.Control.Feedback type="invalid">{`${errors.github_user}`}</BS.Form.Control.Feedback>
                            </BS.Form.Group>
                            <BS.Form.Group className="mb-3">
                                <BS.Form.Label>Github Repo</BS.Form.Label>
                                <Formik.Field
                                    isInvalid={errors.github_repo}
                                    name="github_repo"
                                    as={BS.Form.Control}
                                    type="text"
                                />
                                <BS.Form.Text className="text-muted">
                                    Name of the github repo to read releases
                                    from (TODO: validate)
                                </BS.Form.Text>
                                <BS.Form.Control.Feedback type="invalid">{`${errors.github_repo}`}</BS.Form.Control.Feedback>
                            </BS.Form.Group>
                            <BS.Form.Group>
                                <BS.Form.Control
                                    style={{ maxWidth: "100px" }}
                                    as={BS.Button}
                                    variant="primary"
                                    type="submit"
                                    isInvalid={!!error}
                                >
                                    Update
                                </BS.Form.Control>
                                <BS.Form.Control.Feedback type="invalid">
                                    Update failed: {`${error}`}
                                </BS.Form.Control.Feedback>
                            </BS.Form.Group>
                        </Formik.Form>
                    )}
                </Formik.Formik>
            )}
        </>
    );
}
