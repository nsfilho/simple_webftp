import React from 'react'
import { toast } from 'react-toastify'
import './App.css'

function App() {
    const [file, setFile] = React.useState<File | null>(null);
    const [availablesFiles, setAvailablesFiles] = React.useState<string[]>([]);

    React.useEffect(() => {
        document.title = 'File transfer';
        const handleCheckInterval = setInterval(() => {
            fetch('/files/list')
                .then(response => response.json())
                .then(data => {
                    setAvailablesFiles(data);
                })
                .catch(error => {
                    toast.error(`Error checking files: ${error.message}`);
                });
        }, 1000);
        return () => {
            clearInterval(handleCheckInterval);
        }
    }, []);

    const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const files = event.target.files;
        if (files) {
            setFile(files[0]);
        }
    }

    const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        if (file) {
            const formData = new FormData();
            formData.append('file', file);
            fetch('/files/upload', {
                method: 'POST',
                body: formData
            })
                .then(response => response.json())
                .then(() => {
                    toast.success('File uploaded successfully');
                    setFile(null);
                })
                .catch(error => {
                    toast.error(`Error uploading file: ${error.message}`);
                });
        }
    }

    return (
        <>
            <form onSubmit={handleSubmit}>
                <h3>File upload</h3>
                <div className="container">
                    <div className="upload">
                        <input type="file" onChange={handleFileChange} />
                        {file && <button>Enviar</button>}
                    </div>
                    {file && (
                        <section>
                            <span className="fileDetails">File details:</span>
                            <ul>
                                <li>Name: {file.name}</li>
                                <li>Type: {file.type}</li>
                                <li>Size: {file.size} bytes</li>
                            </ul>
                        </section>
                    )}
                    <div className="files">
                        <h3>Files availables</h3>
                        <ul>
                            {availablesFiles.map((file) => (
                                <li key={file}>{file}</li>
                            ))}
                        </ul>
                    </div>
                </div>
            </form>
        </>
    )
}

export default App;
