terraform {
  required_version = ">=0.12.0"

  backend "s3" {
    bucket = "hyiromori-terraform"
    region = "us-east-1"
    key    = "example-cognito/terraform.tfstate"
  }
}

provider "aws" {
  version = "~>2.18.0"
  region  = "us-east-1"
}

data "aws_caller_identity" "current" {}

locals {
  aws_account_id = data.aws_caller_identity.current.account_id
}
