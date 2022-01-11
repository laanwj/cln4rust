//! Rust Module that contains all the specific version
//! for c-lightning JSON RPC 2.0 call

// Rust JSON-RPC Library
// Written in 2015 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

/// Known lightningd error codes.
///
/// **Keep this up to date with `lightningd/jsonrpc_errors.h`**
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum RpcErrorCode {
    // Standard errors defined by JSON-RPC 2.0 standard
    /// Invalid request
    JSONRPC2_INVALID_REQUEST = -32600,
    /// Method not found
    JSONRPC2_METHOD_NOT_FOUND = -32601,
    /// Invalid parameters
    JSONRPC2_INVALID_PARAMS = -32602,

    /// Uncategorized error
    LIGHTNINGD = -1,

    /// Developer error in the parameters to param() call
    PARAM_DEV_ERROR = -2,

    // Errors from `pay`, `sendpay`, or `waitsendpay` commands
    /// In progress
    PAY_IN_PROGRESS = 200,
    /// `rhash` already used
    PAY_RHASH_ALREADY_USED = 201,
    /// Unparseable onion address
    PAY_UNPARSEABLE_ONION = 202,
    /// Destination permanent failure
    PAY_DESTINATION_PERM_FAIL = 203,
    /// Try another route
    PAY_TRY_OTHER_ROUTE = 204,
    /// Route not found
    PAY_ROUTE_NOT_FOUND = 205,
    /// Route too expensive
    PAY_ROUTE_TOO_EXPENSIVE = 206,
    /// Invoice has expired
    PAY_INVOICE_EXPIRED = 207,
    /// No such payment
    PAY_NO_SUCH_PAYMENT = 208,
    /// Unspecified error
    PAY_UNSPECIFIED_ERROR = 209,
    /// Stopped retrying
    PAY_STOPPED_RETRYING = 210,

    // `fundchannel` or `withdraw` errors
    /// Funding maximum exceeded
    FUND_MAX_EXCEEDED = 300,
    /// Cannot afford funding
    FUND_CANNOT_AFFORD = 301,
    /// Funding output is dust
    FUND_OUTPUT_IS_DUST = 302,

    // Errors from `invoice` command
    /// Invoice label already exists
    INVOICE_LABEL_ALREADY_EXISTS = 900,
    /// Invoice pre-image already exists
    INVOICE_PREIMAGE_ALREADY_EXISTS = 901,
}
