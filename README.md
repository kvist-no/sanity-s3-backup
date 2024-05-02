# Sanity to AWS S3 Backup

This tool allows you to easily create a backup of a Sanity dataset and upload it to an AWS S3 bucket.

You can use easily use this as a cronjob in Kubernetes.

Per now, [due to limitations in the sanity HTTP API](https://www.sanity.io/docs/export), this does not back up the assets, only the documents.

## How to use it

Run the Docker image with the following environment variables:

```env
RUST_LOG=sanity_s3_backup\=info,aws_sdk_s3\=warn
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=
S3_BUCKET=
SANITY_DATASET=
SANITY_PROJECT_ID=
SANITY_TOKEN=
```

### Required permissions

We have the following IAM policy on the group:

```json
{
  "Sid": "VisualEditor0",
  "Effect": "Allow",
  "Action": "s3:PutObject",
  "Resource": "arn:aws:s3:::<BUCKET>/*"
}
```