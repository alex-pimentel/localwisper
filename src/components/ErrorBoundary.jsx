import { Component } from 'react'

export default class ErrorBoundary extends Component {
  constructor(props) {
    super(props)
    this.state = { hasError: false, error: null }
  }

  static getDerivedStateFromError(error) {
    return { hasError: true, error }
  }

  render() {
    if (this.state.hasError) {
      return (
        <div style={{
          display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center',
          height: '100vh', fontFamily: 'system-ui, sans-serif', padding: 24, textAlign: 'center',
        }}>
          <h2>Something went wrong</h2>
          <p style={{ color: '#666', marginBottom: 16 }}>
            An unexpected error occurred. Please reload to try again.
          </p>
          <button onClick={() => window.location.reload()}
            style={{ padding: '8px 16px', borderRadius: 4, border: '1px solid #ccc', cursor: 'pointer' }}>
            Reload
          </button>
        </div>
      )
    }
    return this.props.children
  }
}
