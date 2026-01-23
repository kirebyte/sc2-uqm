# API Contract

Describe the external interfaces. Use OpenAPI or equivalent when ready.

Example skeleton (OpenAPI snippet)
openapi: 3.1.0
info:
  title: [TODO]
  version: 0.1.0
paths:
  /health:
    get:
      summary: Health check
      responses:
        '200':
          description: Service is healthy
