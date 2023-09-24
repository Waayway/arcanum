use std::{ collections::HashMap, future::Future };

use super::{ Param, Type };

/* ################ */
/* #### TRAITS #### */
/* ################ */
pub trait RouteHandler {
    fn handle(&self, params: FnParams) -> (String, Type);
}

type FnParams = HashMap<String, Param>;

/* ################# */
/* #### Structs #### */
/* ################# */

pub struct Route<F> where F: Fn(FnParams) -> String {
    func: F,
    res_type: Type,
}

pub struct ARoute<F, Fut> where F: Fn(FnParams) -> Fut, Fut: Future<Output = String> {
    func: F,
    res_type: Type,
}

pub struct CRoute<F, C> where F: Fn(C, FnParams) -> String, C: Clone {
    func: F,
    res_type: Type,
    controller: C,
}
pub struct ACRoute<F,Fut,C> where F: Fn(C, FnParams) -> Fut, Fut: Future<Output = String>, C: Clone {
    func: F,
    res_type: Type,
    controller: C,
}

impl<F> Route<F>
    where
        F: Fn(FnParams) -> String,
{
    pub fn new(func: F, res_type: Type) -> Box<Route<F>> {
        Box::new(Route {
            func: func,
            res_type,
        })
    }
    pub fn json(func: F) -> Box<Route<F>> {
        Self::new(func, Type::Json)
    }
    pub fn raw(func: F) -> Box<Route<F>> {
        Self::new(func, Type::Raw)
    }
    pub fn html(func: F) -> Box<Route<F>> {
        Self::new(func, Type::Html)
    }
}
impl<F,Fut> ARoute<F,Fut> where F: Fn(FnParams) -> Fut, Fut: Future<Output = String> {
    pub fn new(func: F, res_type: Type) -> Box<ARoute<F,Fut>> {
        Box::new(ARoute {
            func: func,
            res_type,
        })
    }
    pub fn json(func: F) -> Box<ARoute<F,Fut>> {
        Self::new(func, Type::Json)
    }
    pub fn raw(func: F) -> Box<ARoute<F,Fut>> {
        Self::new(func, Type::Raw)
    }
    pub fn html(func: F) -> Box<ARoute<F,Fut>> {
        Self::new(func, Type::Html)
    }
}

impl<F, C> CRoute<F, C> where F: Fn(C, FnParams) -> String, C: Clone {
    pub fn new(func: F, res_type: Type, controller: C) -> Box<CRoute<F, C>> {
        Box::new(CRoute {
            func,
            res_type,
            controller,
        })
    }
    pub fn json(func: F, controller: C) -> Box<CRoute<F, C>> {
        Self::new(func, Type::Json, controller)
    }
    pub fn raw(func: F, controller: C) -> Box<CRoute<F, C>> {
        Self::new(func, Type::Raw, controller)
    }
    pub fn html(func: F, controller: C) -> Box<CRoute<F, C>> {
        Self::new(func, Type::Html, controller)
    }
}

impl<F,Fut,C> ACRoute<F,Fut,C> where F: Fn(C, FnParams) -> Fut, Fut: Future<Output = String>, C: Clone {
    pub fn new(func: F, res_type: Type, controller: C) -> Box<ACRoute<F,Fut,C>> {
        Box::new(ACRoute {
            func,
            res_type,
            controller,
        })
    }
    pub fn json(func: F, controller: C) -> Box<ACRoute<F,Fut,C>> {
        Self::new(func, Type::Json, controller)
    }
    pub fn raw(func: F, controller: C) -> Box<ACRoute<F,Fut,C>> {
        Self::new(func, Type::Raw, controller)
    }
    pub fn html(func: F, controller: C) -> Box<ACRoute<F,Fut,C>> {
        Self::new(func, Type::Html, controller)
    }
}


impl<F> RouteHandler
    for Route<F>
    where
        F: Fn(FnParams) -> String,
{
    fn handle(&self, params: FnParams) -> (String, Type) {
        ((self.func)(params), self.res_type.clone())
    }
}
impl<F,Fut> RouteHandler for ARoute<F,Fut> where F: Fn(FnParams) -> Fut, Fut: Future<Output = String> {
    fn handle(&self, params: FnParams) -> (String, Type) {
        let output = smol::block_on((self.func)(params));
        (output, self.res_type.clone())
    }
}

impl<F, C> RouteHandler for CRoute<F, C> where F: Fn(C, FnParams) -> String, C: Clone {
    fn handle(&self, params: FnParams) -> (String, Type) {
        ((self.func)(self.controller.clone(), params), self.res_type.clone())
    }
}
impl<F,Fut,C> RouteHandler for ACRoute<F,Fut,C> where F: Fn(C, FnParams) -> Fut, Fut: Future<Output = String>, C: Clone {
    fn handle(&self, params: FnParams) -> (String, Type) {
        let output = smol::block_on((self.func)(self.controller.clone(), params));
        (output, self.res_type.clone())
    }
}
